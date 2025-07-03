use std::{
    cell::UnsafeCell,
    error::Error,
    mem::MaybeUninit,
    sync::atomic::{AtomicU8, Ordering},
};

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct Channel<T> {
    state: AtomicU8,
    cell: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Send> Sync for Channel<T> {}

unsafe impl<T: Send> Send for Channel<T> {}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(EMPTY),
            cell: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn send(&self, value: T) -> Result<(), Box<dyn Error>> {
        if self
            .state
            .compare_exchange(EMPTY, WRITING, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            return Err("no more than 1 sent message".into());
        }
        unsafe { &mut *self.cell.get() }.write(value);
        self.state.store(READY, Ordering::Release);
        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        self.state.load(Ordering::Relaxed) == READY
    }

    pub fn recv(&self) -> Result<T, Box<dyn Error>> {
        if self
            .state
            .compare_exchange(READY, READING, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return Err("no more than 1 message".into());
        }
        Ok(unsafe { (*self.cell.get()).assume_init_read() })
    }
}
