use std::rc::Rc;

struct X {
    name: String,
    age: u16,
}

impl X {
    fn new(name: String, age: u16) -> Self {
        Self { name, age }
    }

    fn something(&self) {
        println!("{}", self.name)
    }

    fn something_mut(&mut self) {
        println!("{}", self.name)
    }
}

fn main() {
    println!("{}", std::mem::size_of::<Rc<X>>());
    let a = Rc::new(X::new("YawKar".into(), 22));
    a.something();

    // cannot borrow data in an `Rc` as mutable;
    //trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<X>`
    // a.something_mut();

    let b = Rc::clone(&a);
    b.something();
}
