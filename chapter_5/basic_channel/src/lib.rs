use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

// Compiler can infer that from Mutex and Condvar
// unsafe impl<T: Send> Sync for Channel<T> {}

// Compiler can infer that from Mutex and Condvar
// unsafe impl<T: Send> Send for Channel<T> {}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    pub fn recv(&self) -> T {
        let mut lock = self.queue.lock().unwrap();
        loop {
            if let Some(item) = lock.pop_front() {
                return item;
            }
            lock = self.item_ready.wait(lock).unwrap();
        }
    }

    pub fn send(&self, value: T) {
        let mut lock = self.queue.lock().unwrap();
        lock.push_back(value);
        self.item_ready.notify_one();
    }
}
