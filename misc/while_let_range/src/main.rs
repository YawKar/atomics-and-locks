use std::sync::atomic::{AtomicU8, Ordering};

fn main() {
    const N: u8 = 100;
    let counter = AtomicU8::new(0);
    while let i @ 0..N = counter.load(Ordering::Relaxed) {
        counter.store(i + 1, Ordering::Relaxed);
        println!("{i}")
    }
}
