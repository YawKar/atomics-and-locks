use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Duration,
};

fn main() {
    println!("{}", get_x());
    println!("{}", get_x());
    println!("{}", get_x());
    println!("{}", get_x());
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Ordering::Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Ordering::Relaxed);
    }
    x
}

fn calculate_x() -> u64 {
    let long_time = Duration::from_secs(1);
    thread::sleep(long_time);
    123
}
