use std::sync::atomic::{AtomicU32, Ordering};

use atomic_wait::{wait, wake_all, wake_one};
use mmutex_2::MMutex2Guard;

pub struct CondVar {
    counter: AtomicU32,
}

impl CondVar {
    pub fn new() -> Self {
        Self {
            counter: AtomicU32::new(0),
        }
    }

    pub fn wait<'a, T>(&self, guard: MMutex2Guard<'a, T>) -> MMutex2Guard<'a, T> {
        let counter = self.counter.load(Ordering::Relaxed);

        let m = guard.lock;
        drop(guard);

        wait(&self.counter, counter);

        m.lock()
    }

    pub fn notify_one(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
        wake_one(&self.counter);
    }

    pub fn notify_all(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
        wake_all(&self.counter);
    }
}
