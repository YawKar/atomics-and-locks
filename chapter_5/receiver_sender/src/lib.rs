pub mod my_solution {
    use std::{
        cell::UnsafeCell,
        marker::PhantomData,
        ops::{Deref, DerefMut},
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
    };

    struct SpinLock<T> {
        lock: AtomicBool,
        value: UnsafeCell<T>,
    }

    unsafe impl<T: Send> Sync for SpinLock<T> {}

    unsafe impl<T: Send> Send for SpinLock<T> {}

    impl<T> SpinLock<T> {
        fn new(value: T) -> Self {
            Self {
                lock: AtomicBool::new(false),
                value: UnsafeCell::new(value),
            }
        }

        fn lock(&self) -> SpinLockGuard<'_, T> {
            while self.lock.swap(true, Ordering::Acquire) {
                std::hint::spin_loop();
            }
            SpinLockGuard {
                lock: self,
                value: PhantomData,
            }
        }

        fn unlock(&self) {
            self.lock.store(false, Ordering::Release);
        }
    }

    struct SpinLockGuard<'a, T> {
        lock: &'a SpinLock<T>,
        value: PhantomData<&'a mut T>,
    }

    impl<T> Drop for SpinLockGuard<'_, T> {
        fn drop(&mut self) {
            self.lock.unlock();
        }
    }

    impl<T> Deref for SpinLockGuard<'_, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.lock.value.get() }
        }
    }

    impl<T> DerefMut for SpinLockGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut *self.lock.value.get() }
        }
    }

    pub struct Receiver<T> {
        value: Arc<SpinLock<Option<T>>>,
    }

    impl<T> Receiver<T> {
        pub fn is_ready(&self) -> bool {
            self.value.lock().is_some()
        }

        pub fn receive(self) -> Option<T> {
            self.value.lock().take()
        }
    }

    pub struct Sender<T> {
        value: Arc<SpinLock<Option<T>>>,
    }

    impl<T> Sender<T> {
        pub fn send(self, value: T) {
            self.value.lock().replace(value);
        }
    }

    pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
        let spinlock = Arc::new(SpinLock::new(None));
        let receiver = Receiver {
            value: Arc::clone(&spinlock),
        };
        let sender = Sender {
            value: Arc::clone(&spinlock),
        };
        (sender, receiver)
    }
}
