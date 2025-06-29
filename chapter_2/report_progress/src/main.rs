use std::{
    io::{self, Write},
    sync::atomic::{AtomicU8, Ordering},
    thread,
    time::Duration,
};

fn main() {
    let progress = AtomicU8::new(0);
    let main_thread = thread::current();
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for i in 1..=25 {
                    process(i);
                    progress.fetch_add(1, Ordering::Relaxed);
                    main_thread.unpark();
                }
            });
        }
        loop {
            let completed = progress.load(Ordering::Relaxed);
            print!("\rProgress: {}%...", progress.load(Ordering::Relaxed));
            io::stdout().flush().unwrap();
            if completed == 100 {
                print!("\n");
                break;
            }
            thread::park_timeout(Duration::from_secs(1));
        }
    });
    println!("Done!");
}

fn process(_i: i32) {
    thread::sleep(Duration::from_millis(50));
}
