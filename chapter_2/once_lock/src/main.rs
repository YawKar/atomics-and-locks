use std::sync::OnceLock;

fn main() {
  let once_lock: OnceLock<usize> = OnceLock::new();
  println!("{:?}", once_lock.get());
  once_lock.get_or_init(|| {
    123
  });
  println!("{:?}", once_lock.get());
}
