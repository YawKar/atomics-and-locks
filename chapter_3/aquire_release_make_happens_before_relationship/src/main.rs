use std::{
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    thread,
    time::Duration,
};

fn main() {
    static VALUE: AtomicU64 = AtomicU64::new(0);
    static READY: AtomicBool = AtomicBool::new(false);
    thread::scope(|s| {
        s.spawn(|| {
            VALUE.store(42, Ordering::Relaxed);
            READY.store(true, Ordering::Release);
        });

        while !READY.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(100));
            println!("waiting...")
        }
        println!("{}", VALUE.load(Ordering::Relaxed));
    });
}
