use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

static mut DATA: u64 = 0;
static LOCKED: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(f);
        }
    });
    unsafe {
        let x = DATA;
        println!("{x}");
    }
}

fn f() {
    if LOCKED
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        unsafe {
            // We have taken an exclusive lock
            DATA += 1;
        }
        LOCKED.store(false, Ordering::Release);
    }
}
