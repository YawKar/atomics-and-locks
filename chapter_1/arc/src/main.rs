use std::{sync::Arc, thread};

struct X {
    name: String, 
    age: u16,
}

impl X {
    fn new(name: String, age: u16) -> Self {
        Self {
            name,
            age,
        }
    }

    fn do_something(&self) {
        println!("{}", self.name)
    }

    fn do_something_mut(&mut self) {
        println!("{}", self.age)
    }
}

fn main() {
    println!("{}", std::mem::size_of::<Arc<X>>());
    let a = Arc::new(X::new("YawKar".into(), 22));
    let h = thread::spawn({
        let a = a.clone();
        move || {
            // cannot borrow data in an `Arc` as mutable
            // trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<X>`
            // a.do_something_mut();
            a.do_something();
        }
    });
    a.do_something();
    h.join().unwrap();
}
