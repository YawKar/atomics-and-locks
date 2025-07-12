use std::{thread, time::Duration};

use better_condvar::CondVar;
use mmutex_2::MMutex2;

fn main() {
    let condvar = CondVar::new();
    let m = MMutex2::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            let mut guard = m.lock();
            let mut wake_ups = 0;
            while *guard < 100 {
                guard = condvar.wait(guard);
                wake_ups += 1;
            }
            println!("It is {}, {wake_ups}", *guard);
        });

        s.spawn(|| {
            thread::sleep(Duration::from_secs(1));
            for _ in 0..101 {
                *m.lock() += 1;
            }
            condvar.notify_all();
        });
    });
}
