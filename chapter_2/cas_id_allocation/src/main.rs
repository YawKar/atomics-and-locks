use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

fn main() {
    thread::scope(|s| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            s.spawn(|| {
                println!("{}", allocate_id());
                println!("{}", allocate_id());
                println!("{}", allocate_id());
            });
        }
    });
}

fn allocate_id() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let mut id = NEXT.load(Ordering::Relaxed);
    loop {
        assert!(id < u64::MAX, "too many IDs!");
        match NEXT.compare_exchange(
            id,
            id.saturating_add(1),
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => break id,
            Err(v) => id = v,
        }
    }
}

fn allocate_id_one_liner() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    NEXT.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
        .expect("Too many IDs")
}
