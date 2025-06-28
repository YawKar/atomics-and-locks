use std::{cell::Cell, thread};

fn main() {
    // all primitive values such as i32, bool, str are Send+Sync
    let mut a = 14;
    let mut c = Cell::new(5);
    thread::scope(|s| {
        s.spawn(|| {
            a = 1;
            *c.get_mut() += 1;
            println!("thread: {a}");
        });
    });
    println!("{}", a);
    println!("{}", c.get());
}
