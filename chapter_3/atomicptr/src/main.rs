use std::sync::atomic::{AtomicPtr, Ordering};

fn main() {
    println!("{:#?}", get_data());
}

#[derive(Debug)]
struct Data {}

impl Data {
    fn new() -> Self {
        Self {}
    }
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Ordering::Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(Data::new()));
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(),
            p,
            Ordering::Release,
            Ordering::Acquire,
        ) {
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    unsafe { &*p }
}
