use std::sync::atomic::{AtomicU8, Ordering};

fn main() {
    let au8 = AtomicU8::new(0);
    au8.store(4, Ordering::Relaxed);
    println!("{}", au8.load(Ordering::Relaxed));
}
