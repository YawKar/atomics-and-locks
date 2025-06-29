use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Duration,
};

fn main() {
    let x = AtomicU64::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            let a = x.load(Ordering::Relaxed);
            thread::sleep(Duration::from_micros(10));
            let b = x.load(Ordering::Relaxed);
            thread::sleep(Duration::from_micros(10));
            let c = x.load(Ordering::Relaxed);
            thread::sleep(Duration::from_micros(10));
            let d = x.load(Ordering::Relaxed);
            println!("{a} {b} {c} {d}");
        });
        s.spawn(|| {
            x.fetch_add(5, Ordering::Relaxed);
            thread::sleep(Duration::from_micros(10));
            x.fetch_add(10, Ordering::Relaxed);
        });
    });
}
