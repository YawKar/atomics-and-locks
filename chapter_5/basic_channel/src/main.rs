use std::thread;

use basic_channel::Channel;

fn main() {
    let ch = Channel::new();

    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(|| {
                for i in 0..100 {
                    ch.send(i);
                }
            });
        }
        for _ in 0..100 {
            s.spawn(|| {
                for _ in 0..100 {
                    ch.recv();
                }
            });
        }
    });

    assert_eq!(0, ch.len());
}
