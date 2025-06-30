use std::{
    sync::atomic::{AtomicBool, AtomicU64, Ordering, fence},
    thread,
};

static DATA: [AtomicU64; 5] = [
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
];

static FLAG: AtomicBool = AtomicBool::new(false);

fn main() {
    let producer = thread::spawn(|| {
        for i in 0..DATA.len() {
            DATA[i].store(i as u64, Ordering::Relaxed);
        }

        fence(Ordering::Release);
        // Effectively a Release operation
        FLAG.store(true, Ordering::Relaxed);
    });

    let consumer = thread::spawn(|| {
        // Effectively an Acquire operation
        while !FLAG.load(Ordering::Relaxed) {}
        fence(Ordering::Acquire);

        for i in 0..DATA.len() {
            println!("{}", DATA[i].load(Ordering::Relaxed));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
