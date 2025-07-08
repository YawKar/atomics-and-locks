use std::thread;

use mmutex_2_training::MMutex2;

fn main() {
    let m = MMutex2::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..100_000 {
                *m.lock() += 1;
            }
        });
        s.spawn(|| {
            for _ in 0..200_000 {
                *m.lock() += 1;
            }
        });
    });
    println!("{}", *m.lock());
}
