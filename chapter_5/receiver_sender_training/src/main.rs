use std::{thread, time::Duration};

use receiver_sender_training::channel;

fn main() {
    let (sender, receiver) = channel::<i32>();
    thread::scope(|s| {
        s.spawn(|| {
            thread::sleep(Duration::from_secs(1));
            sender.send(123);
        });
        s.spawn(|| {
            println!("{}", receiver.receive());
        });
    });
    let (sender, receiver) = channel::<i32>();
    thread::scope(|s| {
        s.spawn(|| {
            sender.send(123);
        });
        s.spawn(|| {
            thread::sleep(Duration::from_secs(1));
            println!("{}", receiver.receive());
        });
    });
}
