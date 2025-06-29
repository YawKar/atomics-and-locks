use std::{sync::atomic::{AtomicU64, Ordering}, thread};

use rand::RngCore;

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(|| {
                println!("Got key: {}", get_key());
            });
        }
    });
}

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let mut key = KEY.load(Ordering::Relaxed);
    if key == 0 {
        key = rand::rng().next_u64();
        match KEY.compare_exchange(0, key, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => key,
            Err(other_key) => other_key,
        }
    } else {
        key
    }
}
