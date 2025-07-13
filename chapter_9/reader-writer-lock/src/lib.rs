use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

use atomic_wait::{wait, wake_all, wake_one};

const STATE_WRITE_LOCKED: u32 = u32::MAX;

pub struct RwLock<T> {
    // number of readers, or u32::MAX if write-locked
    state: AtomicU32,
    value: UnsafeCell<T>,
}

impl<T> RwLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn read(&self) -> ReadGuard<T> {
        let mut readers = self.state.load(Ordering::Relaxed);
        loop {
            if readers < STATE_WRITE_LOCKED {
                assert!(readers < STATE_WRITE_LOCKED - 1, "too many readers");
                match self.state.compare_exchange_weak(
                    readers,
                    readers + 1,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        return ReadGuard {
                            lock: &self,
                            _value: PhantomData,
                        };
                    }
                    Err(e) => readers = e,
                }
            }
            if readers == STATE_WRITE_LOCKED {
                wait(&self.state, STATE_WRITE_LOCKED);
                readers = self.state.load(Ordering::Relaxed);
            }
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        while let Err(state) =
            self.state
                .compare_exchange(0, STATE_WRITE_LOCKED, Ordering::Acquire, Ordering::Relaxed)
        {
            wait(&self.state, state);
        }
        WriteGuard {
            lock: &self,
            _value: PhantomData,
        }
    }
}

pub struct ReadGuard<'a, T> {
    lock: &'a RwLock<T>,
    _value: PhantomData<&'a T>,
}

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> Drop for ReadGuard<'_, T> {
    fn drop(&mut self) {
        if self.lock.state.fetch_sub(1, Ordering::Release) == 1 {
            wake_one(&self.lock.state);
        }
    }
}

pub struct WriteGuard<'a, T> {
    lock: &'a RwLock<T>,
    _value: PhantomData<&'a mut T>,
}

impl<T> Deref for WriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.store(0, Ordering::Release);
        wake_all(&self.lock.state);
    }
}
