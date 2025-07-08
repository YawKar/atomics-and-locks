use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

use atomic_wait::{wait, wake_one};

const STATE_FREE: u32 = 0;
const STATE_LOCKED: u32 = 1;
const STATE_WAITED: u32 = 2;

pub struct MMutex2<T> {
    state: AtomicU32,
    cell: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for MMutex2<T> {}

unsafe impl<T: Send> Send for MMutex2<T> {}

impl<T> MMutex2<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(STATE_FREE),
            cell: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn lock(&self) -> MMutex2Guard<T> {
        if self
            .state
            .compare_exchange(
                STATE_FREE,
                STATE_LOCKED,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_err()
        {
            self.lock_contention();
        }

        MMutex2Guard {
            m: self,
            _value: PhantomData,
        }
    }

    // Either STATE_LOCK or STATE_WAITED
    #[cold]
    fn lock_contention(&self) {
        let mut spin = 0;

        // What if we can spin for a little and quickly acquire the lock
        while self.state.load(Ordering::Relaxed) == STATE_LOCKED && spin < 100 {
            spin += 1;
            std::hint::spin_loop();
        }

        // Last try
        if self
            .state
            .compare_exchange(
                STATE_FREE,
                STATE_LOCKED,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            return;
        }

        // Then we go sleep
        while self.state.swap(2, Ordering::Acquire) != 0 {
            wait(&self.state, 2);
        }
    }

    #[inline]
    fn unlock(&self) {
        if self.state.swap(STATE_FREE, Ordering::Release) == STATE_WAITED {
            wake_one(&self.state);
        }
    }
}

pub struct MMutex2Guard<'a, T> {
    m: &'a MMutex2<T>,
    _value: PhantomData<&'a mut T>,
}

impl<T> Drop for MMutex2Guard<'_, T> {
    fn drop(&mut self) {
        self.m.unlock();
    }
}

impl<T> Deref for MMutex2Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.m.cell.get() }
    }
}

impl<T> DerefMut for MMutex2Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.m.cell.get() }
    }
}
