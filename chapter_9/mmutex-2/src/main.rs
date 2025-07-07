use std::thread;

use mmutex_2::MMutex2;

fn main() {
    let m = MMutex2::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..100_000 {
                let mut l = m.lock();
                *l += 1;
            }
        });
        s.spawn(|| {
            for _ in 0..100_000 {
                let mut l = m.lock();
                *l += 1;
            }
        });
    });
    println!("{}", *m.lock());
}
