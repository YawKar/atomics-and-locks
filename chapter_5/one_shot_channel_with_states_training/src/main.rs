use std::thread;

use one_shot_channel_with_states_training::Channel;

fn main() {
    let ch = Channel::new();
    thread::scope(|s| {
        s.spawn(|| {
            ch.send(123).unwrap();
        });
        s.spawn(|| {
            while !ch.is_ready() {
                std::hint::spin_loop();
            }
            println!("{}", ch.recv().unwrap());
        });
    });
}
