use std::{
    cell::UnsafeCell, mem::MaybeUninit, sync::atomic::{fence, AtomicBool, Ordering}, thread, time::Duration
};

// It is bad
pub struct OneShot<T> {
    ready: AtomicBool,
    item: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Send> Sync for OneShot<T> {}

impl<T> OneShot<T> {
    pub fn new() -> Self {
        Self {
            ready: AtomicBool::new(false),
            item: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn send(&self, value: T) -> Result<(), &str> {
        if self.ready.swap(true, Ordering::Relaxed) {
            return Err("It was already set");
        }
        unsafe { &mut *self.item.get() }.write(value);
        fence(Ordering::Release);
        Ok(())
    }

    pub fn recv(self) -> T {
        while !self.ready.load(Ordering::Acquire) {
            std::hint::spin_loop();
        }
        unsafe { self.item.into_inner().assume_init() }
    }
}
