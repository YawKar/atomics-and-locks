use std::{sync::Mutex, thread, time::Duration};

use basic_reference_counting::Arc;

#[derive(Debug)]
struct X {
    value: i32,
}

impl Drop for X {
    fn drop(&mut self) {
        println!("Dropped")
    }
}

fn main() {
    let a = Arc::new(Mutex::new(X { value: 123 }));

    thread::scope(|s| {
        s.spawn(|| {
            let mut lock = a.lock().unwrap();
            lock.value = 213;
        });

        for _ in 0..2 {
            let lock = a.lock().unwrap();
            println!("{:?}", *lock);
            drop(lock);
            thread::sleep(Duration::from_secs(1));
        }
    });
}
