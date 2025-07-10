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

    pub fn lock(&self) -> MMutex2Guard<T> {
        // Fast case.
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
            // Slow case.
            self.lock_contented();
        }
        MMutex2Guard {
            lock: &self,
            _value: PhantomData,
        }
    }

    fn lock_contented(&self) {
        let mut spin = 0;
        // Small spin just to try to lock it.
        while self.state.load(Ordering::Relaxed) == 1 && spin < 100 {
            spin += 1;
            std::hint::spin_loop();
        }

        // Last chance.
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

        while self.state.swap(STATE_WAITED, Ordering::Acquire) != STATE_FREE {
            wait(&self.state, STATE_WAITED);
        }
    }

    fn unlock(&self) {
        if self.state.swap(STATE_FREE, Ordering::Release) == STATE_WAITED {
            wake_one(&self.state);
        }
    }
}

pub struct MMutex2Guard<'a, T> {
    lock: &'a MMutex2<T>,
    _value: PhantomData<&'a mut T>,
}

impl<T> Drop for MMutex2Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

impl<T> Deref for MMutex2Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.cell.get() }
    }
}

impl<T> DerefMut for MMutex2Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.cell.get() }
    }
}
