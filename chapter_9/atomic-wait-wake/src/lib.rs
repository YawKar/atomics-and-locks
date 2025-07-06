use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU32, Ordering},
};

use atomic_wait::{wait, wake_one};

pub struct MMutex<T> {
    state: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for MMutex<T> {}

unsafe impl<T: Send> Send for MMutex<T> {}

impl<T> MMutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MMutexGuard<T> {
        loop {
            while self.state.load(Ordering::Acquire) == 1 {
                wait(&self.state, 1);
            }
            if self.state.swap(1, Ordering::Release) == 0 {
                // Locked!
                break;
            }
        }
        MMutexGuard {
            lock: self,
            value: PhantomData,
        }
    }

    fn unlock(&self) {
        self.state.store(0, Ordering::Release);
        wake_one(&self.state);
    }
}

pub struct MMutexGuard<'a, T> {
    lock: &'a MMutex<T>,
    value: PhantomData<&'a mut T>,
}

impl<T> Drop for MMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

impl<T> Deref for MMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for MMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}
