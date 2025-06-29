use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    println!("{}", allocate_id());
    println!("{}", allocate_id());
    println!("{}", allocate_id());
}

fn allocate_id() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    if id >= 1000 {
        NEXT.fetch_sub(1, Ordering::Relaxed);
        panic!("too many IDs!");
    }
    id
}
