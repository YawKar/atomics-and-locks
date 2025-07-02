use std::thread;

use basic_channel_train::Channel;

fn main() {
    let ch = Channel::new();
    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                ch.send(i).unwrap()
            }
        });
        s.spawn(|| {
            for _ in 0..100 {
                let value = ch.recv().unwrap();
                println!("{value}");
            }
        });
    });
}
