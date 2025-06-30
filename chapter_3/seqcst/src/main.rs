use std::{
    sync::{
        RwLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

fn main() {
    let string = RwLock::new(String::new());
    thread::scope(|s| {
        s.spawn(|| {
            A.store(true, Ordering::SeqCst);
            if !B.load(Ordering::SeqCst) {
                string.write().unwrap().push('!');
            }
        });

        s.spawn(|| {
            B.store(true, Ordering::SeqCst);
            if !A.load(Ordering::SeqCst) {
                string.write().unwrap().push('!');
            }
        });
    });
    println!("{}", string.read().unwrap());
}
