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
    value: UnsafeCell<MaybeUninit<T>>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(EMPTY),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn send(&self, value: T) -> Result<(), Box<dyn Error>> {
        if self
            .state
            .compare_exchange(EMPTY, WRITING, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            return Err("one shot channel is capable only of 1 message to be sent".into());
        }
        (unsafe { &mut *self.value.get() }).write(value);
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
            panic!("no message to receive!");
        }
        Ok(unsafe { (&mut *self.value.get()).assume_init_read() })
    }
}

unsafe impl<T: Send> Sync for Channel<T> {}
