use std::{
    sync::{Arc, Mutex, atomic::AtomicBool},
    thread,
    time::Duration,
};

fn main() {
    let mutex = Arc::new(Mutex::new(Some(42)));
    let flag = Arc::new(AtomicBool::new(false));
    let handler = thread::spawn({
        let flag = Arc::clone(&flag);
        let mutex = Arc::clone(&mutex);
        move || {
            while !flag.load(std::sync::atomic::Ordering::Relaxed) {}
            println!("Taking a lock from h1");
            let _unused = mutex.lock().unwrap();
            println!("Finally able to print");
        }
    });
    // let result = *mutex.lock().unwrap();
    // if let Some(answer) = result {
    if let Some(answer) = *mutex.lock().unwrap() {
        flag.store(true, std::sync::atomic::Ordering::Relaxed);
        println!("answer: {answer}");
        for _ in 0..5 {
            println!("Doing something in the main thread");
            thread::sleep(Duration::from_millis(500));
        }
    }
    handler.join().unwrap();
}
