use std::{collections::VecDeque, sync::Mutex, thread, time::Duration};

fn main() {
    let queue = Mutex::new(VecDeque::<i32>::new());
    thread::scope(|s| {
        let t = s.spawn(|| {
            loop {
                let item = queue.lock().unwrap().pop_front();
                if let Some(value) = item {
                    dbg!(value);
                } else {
                    thread::park();
                }
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
