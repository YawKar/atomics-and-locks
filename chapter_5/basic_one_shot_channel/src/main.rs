use std::thread;

use basic_one_shot_channel::OneShot;

fn main() {
    let ch = OneShot::new();

    thread::scope(|s| {
        s.spawn(|| {
            ch.send(5).unwrap();
        });
    });

    println!("{}", ch.recv());
}
