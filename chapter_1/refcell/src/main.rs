use std::cell::RefCell;

fn main() {
    // 16 = 8 bytes on BorrowFlag + 4 bytes on i32 + 4 bytes of alignment
    println!("{}", std::mem::size_of::<RefCell<i32>>());
    let rc1 = RefCell::new(5);
    let kek = rc1.borrow_mut();
    drop(kek); // necessary or it won't compile
    *rc1.borrow_mut() = 55;
    *rc1.borrow_mut() = 77;
    println!("{}", rc1.borrow());
    let rc2 = rc1.clone();
    *rc2.borrow_mut() = 123;
    println!("{} {}", rc1.borrow(), rc2.borrow());
}
