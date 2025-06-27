use std::{sync::Mutex, thread, time::{self, Duration}};

fn main() {
    let m = Mutex::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..500 {
                let mut lock = m.lock().unwrap();
                *lock += 1;
                drop(lock)
            }
        });
        s.spawn(|| {
            for _ in 0..500 {
                let mut lock = m.lock().unwrap();
                *lock -= 1;
                drop(lock)
            }
        });
        s.spawn(|| {
            let start = time::Instant::now();
            loop {
                if let Ok(lock) = m.lock() {
                    if *lock != 0 {
                        println!("{}", *lock);
                    }
                }
                if start.elapsed() >= Duration::from_secs(1) {
                    break;
                }
            }
        });
    });
    assert_eq!(*m.lock().unwrap(), 0);
}
