use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    X.store(1, Ordering::Relaxed);
    let t = thread::spawn(f);
    X.store(2, Ordering::Relaxed);
    t.join().unwrap();
    X.store(3, Ordering::Relaxed);
}

fn f() {
    let x = X.load(Ordering::Relaxed);
    assert!(x == 1 || x == 2);
}
