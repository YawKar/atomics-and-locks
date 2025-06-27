use std::thread;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    thread::scope(|s| {
        s.spawn(|| {
            println!("The vector length is: {}", v.len());
        });
        s.spawn(|| {
            for &n in &v {
                println!("{n}");
            }
        });
    });
}
