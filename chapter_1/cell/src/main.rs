use std::{cell::Cell, thread};

fn main() {
    println!("{}", std::mem::size_of::<Cell<i32>>());
    // interior mutability
    let mut c1 = Cell::new(5);
    // let mutableref = c1.get_mut();
    println!("{c1:#?}");
    c1.set(1);
    println!("{c1:#?}");
    let c2 = &c1;
    c2.set(2);
    println!("c1={} c2={}", c1.get(), c2.get());
    // *mutableref = 7;
    // Cell<i32> is not Sync
    // &Cell<i32> is not Send
    // thread::spawn(|| {
    //     c1.set(4);
    //     println!("{c1:#?}");
    // }).join().unwrap();
    thread::spawn(move || {
        c1.set(4);
        println!("{c1:#?}");
    })
    .join()
    .unwrap();
}
