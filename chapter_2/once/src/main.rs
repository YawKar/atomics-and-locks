use std::{sync::Once, thread, time::Duration};

fn main() {
    let h1 = thread::spawn(|| {
        println!("{}", get_cached_value());
    });
    let h2 = thread::spawn(|| {
        println!("{}", get_cached_value());
    });

    h1.join().unwrap();
    h2.join().unwrap();
}

fn get_cached_value() -> i32 {
    static mut VAL: i32 = 0;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        thread::sleep(Duration::from_secs(1));
        unsafe {
            VAL = 123;
        }
    });
    unsafe { VAL }
}
