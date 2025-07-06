use std::sync::atomic::AtomicI32;

#[cfg(not(target_os = "linux"))]
compile_error!("Linux only. Sorry!");

pub fn wait(a: &AtomicI32, expected: u32) {
    unsafe {
        libc::syscall(
            libc::SYS_futex,
            a as *const AtomicI32,
            libc::FUTEX_WAIT,
            expected,
            std::ptr::null::<libc::timespec>(),
        );
    }
}

pub fn wake_one(a: &AtomicI32) {
    unsafe {
        libc::syscall(libc::SYS_futex, a as *const AtomicI32, libc::FUTEX_WAKE, 1);
    }
}
