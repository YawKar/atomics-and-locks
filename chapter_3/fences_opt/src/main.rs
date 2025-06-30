use std::{
    sync::atomic::{AtomicBool, Ordering, fence},
    thread,
};

static mut DATA: [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            unsafe { DATA[i] = i as u64 };
            READY[i].store(true, Ordering::Release);
        });
    }
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Ordering::Relaxed));
    if ready.contains(&true) {
        fence(Ordering::Acquire);
        for i in 0..10 {
            if ready[i] {
                unsafe { println!("{i}: {}", DATA[i]) };
            }
        }
    }
}
