use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
    time::Duration,
};

use futexes::{wait, wake_one};

fn main() {
    let a = AtomicI32::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            thread::sleep(Duration::from_secs(3));
            a.store(1, Ordering::Relaxed);
            wake_one(&a);
        });

        println!("Waiting!");
        while a.load(Ordering::Relaxed) == 0 {
            wait(&a, 0);
        }
        println!("Done!");
    });
}
