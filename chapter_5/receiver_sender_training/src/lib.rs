use std::{
    mem::MaybeUninit,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
};

use atomic_wait::{wait, wake_all};

struct Inner<T> {
    cell: Mutex<MaybeUninit<T>>,
    state: AtomicU32,
}

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T: Send> Sender<T> {
    pub fn send(self, value: T) {
        let mut lock = self.inner.cell.lock().expect("Expected to acquire a lock");
        lock.write(value);
        drop(lock);
        self.inner.state.store(1, Ordering::Release);
        wake_all(&self.inner.state);
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T: Send> Receiver<T> {
    pub fn receive(self) -> T {
        while self.inner.state.load(Ordering::Acquire) == 0 {
            wait(&self.inner.state, 0);
        }
        unsafe { self.inner.cell.lock().unwrap().assume_init_read() }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let i = Arc::new(Inner {
        cell: Mutex::new(MaybeUninit::uninit()),
        state: AtomicU32::new(0),
    });
    (
        Sender {
            inner: Arc::clone(&i),
        },
        Receiver {
            inner: Arc::clone(&i),
        },
    )
}
