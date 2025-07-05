use std::{
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{fence, AtomicUsize, Ordering},
};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

impl<T> ArcData<T> {
    pub fn new(value: T) -> Self {
        Self {
            ref_count: AtomicUsize::new(1),
            data: value,
        }
    }
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Sync for Arc<T> {}
unsafe impl<T: Send + Sync> Send for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(value: T) -> Self {
        Self {
            ptr: NonNull::new(Box::into_raw(Box::new(ArcData::new(value)))).unwrap(),
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.data().ref_count.load(Ordering::Relaxed) == 1 {
            fence(Ordering::Acquire);
            return unsafe { Some(&mut self.ptr.as_mut().data) }
        }
        None
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Self { ptr: self.ptr }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data().data
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        let reference = self.data();
        if reference.ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            drop(unsafe { Box::from_raw(self.ptr.as_ptr()) });
        }
    }
}
