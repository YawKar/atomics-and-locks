use std::{cell::Cell, marker::PhantomData, thread};

fn main() {
    println!("Hello, world!");
    let x = X {
        handle: 32,
        _not_sync: PhantomData,
    };
    // `Cell<()>` cannot be shared between threads safely
    //  within `X`, the trait `Sync` is not implemented for `Cell<()>`
    // thread::scope(|s| {
    //     s.spawn(|| {
    //         println!("{:#?}", x);
    //     });
    // });
}

#[derive(Debug)]
struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>, // makes X !Sync
}
