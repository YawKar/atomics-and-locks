use std::thread;

fn main() {
    let h1 = thread::Builder::new()
        .name("Child 1".into())
        .spawn(child).unwrap();
    let h2 = thread::Builder::new()
        .name("Child 2".into())
        .spawn(child).unwrap();
    println!("Main thread: {:#?}", thread::current());
    h1.join().unwrap();
    h2.join().unwrap();
}

fn child() {
    println!("Child's thread: {:#?}", thread::current());
}
