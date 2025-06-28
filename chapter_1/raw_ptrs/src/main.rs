fn main() {}

struct X {
    p: *mut i32, // is neither Sync nor Send
}

// `trust me compiler`
unsafe impl Send for X {}
unsafe impl Sync for X {}
