use std::thread;

static IMMUTABLE: [i32; 3] = [1, 2, 3];
// static mut MUTABLE_VEC: Vec<u8> = vec![1, 2, 3]; // no allocation
static mut MUTABLE: [i32; 3] = [1, 2, 3];

static FIB: u32 = fib(23);

const fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        mut n => {
            let (mut f0, mut f1) = (0, 1);
            while n > 1 {
                (f0, f1) = (f1, f0 + f1);
                n -= 1;
            }
            f1
        }
    }
}

fn main() {
    println!("{FIB}");
    unsafe {
        let bring_it = &raw mut MUTABLE;
        (*bring_it)[0] += 1;
        println!("{:#?}", &MUTABLE[..]);
    }
    println!("{:#?}", IMMUTABLE);
    let h1 = thread::spawn(|| unsafe {
        MUTABLE = [0, 87, 3];
        println!("{:#?}", &MUTABLE[..]);
    });
    let h2 = thread::spawn(|| unsafe {
        MUTABLE = [1, 8, 4];
        println!("{:#?}", &MUTABLE[..]);
    });
    h1.join().unwrap();
    h2.join().unwrap();
}
