use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let queue = Mutex::new(VecDeque::<i32>::new());
    let not_empty = Condvar::new();
    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut qguard = queue.lock().unwrap();
                let item = loop {
                    if let Some(v) = qguard.pop_front() {
                        break v;
                    } else {
                        qguard = not_empty.wait(qguard).unwrap();
                    }
                };
                dbg!(item);
                drop(qguard);
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_all();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
