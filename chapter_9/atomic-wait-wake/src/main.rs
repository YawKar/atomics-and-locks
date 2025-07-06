use std::thread;

use atomic_wait_wake::MMutex;

fn main() {
    let m = MMutex::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..100_000 {
                let mut lock = m.lock();
                *lock += 1;
            }
        });
        s.spawn(|| {
            for _ in 0..100_000 {
                let mut lock = m.lock();
                *lock += 1;
            }
        });
    });
    println!("{}", *m.lock())
}
