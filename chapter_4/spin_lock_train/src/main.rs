use std::thread;

use spin_lock_train::SpinLock;

fn main() {
    let spin_lock = SpinLock::new(0);
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(|| {
                let mut x = spin_lock.lock();
                *x += 1;
            });
        }
    });
    println!("{}", *spin_lock.lock());

    let spin_lock2 = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| spin_lock2.lock().push(1));

        s.spawn(|| {
            let mut g = spin_lock2.lock();
            g.push(2);
            g.push(2);
        });
    });
    println!("{:?}", spin_lock2.lock().as_slice());
}
