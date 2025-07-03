use std::thread;

use receiver_sender::my_solution::channel;

fn main() {
    let (sender, receiver) = channel();

    thread::scope(|s| {
        let t = thread::current();

        s.spawn(move || {
            sender.send(123);
            t.unpark();
            // sender.send(123);
        });

        while !receiver.is_ready() {
            thread::park();
        }
        println!("{}", receiver.receive().unwrap());
        // println!("{}", receiver.receive().unwrap());
    });
}
