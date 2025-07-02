use std::{
    collections::VecDeque,
    error::Error,
    sync::{Condvar, Mutex},
};

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, value: T) -> Result<(), Box<dyn Error + '_>> {
        self.queue.lock()?.push_back(value);
        self.item_ready.notify_one();
        Ok(())
    }

    pub fn recv(&self) -> Result<T, Box<dyn Error + '_>> {
        let mut lock = self.queue.lock()?;
        while lock.len() == 0 {
            lock = self.item_ready.wait(lock)?;
        }
        Ok(lock
            .pop_front()
            .expect("expected at least 1 element in the queue"))
    }
}
