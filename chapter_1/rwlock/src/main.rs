use std::{sync::RwLock, thread, time::Duration};

fn main() {
    let rwlock = RwLock::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..2000 {
                let mut guard = rwlock.write().unwrap();
                *guard += 1;
            }
        });
        s.spawn(|| {
            for _ in 0..2000 {
                let mut guard = rwlock.write().unwrap();
                *guard -= 1;
            }
        });
        s.spawn(|| {
            let start = std::time::Instant::now();
            loop {
                if let Ok(guard) = rwlock.read() {
                    if *guard != 0 {
                        println!("{}", *guard);
                    }
                }
                if start.elapsed() >= Duration::from_secs(1) {
                    break;
                }
            }
        });
    });
    assert_eq!(*rwlock.read().unwrap(), 0);
}
