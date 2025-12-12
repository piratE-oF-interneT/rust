use std::{cell::Cell, clone, ops::Deref};

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}
#[derive(Debug)]
pub struct MyRc<T> {
    ptr: *mut RcInner<T>,
}

impl<T> MyRc<T> {
    pub fn new(val: T) -> Self {
        let inner = RcInner {
            value: val,
            refcount: Cell::new(1),
        };

        let layout = std::alloc::Layout::new::<RcInner<T>>();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut RcInner<T> };
        unsafe {
            std::ptr::write(ptr, inner);
        }

        Self { ptr: ptr }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        // increase the refcnt and returns the copy of pointer not the copy of heap data

        let inner = unsafe { &*self.ptr };

        inner.refcount.set(inner.refcount.get() + 1);
        let cnt = inner.refcount.get();

        println!("heap value cloned and ref cnt is {}", cnt);

        MyRc { ptr: self.ptr }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let inner = unsafe { &*self.ptr };

        let cnt = inner.refcount.get() - 1;

        if cnt > 0 {
            inner.refcount.set(cnt);
            println!("reference cnt is  : {}", cnt);
        } else {
            // drop value and then deallocate memory
            unsafe { std::ptr::drop_in_place(&mut *self.ptr) };
            let layout = std::alloc::Layout::new::<RcInner<T>>();
            unsafe { std::alloc::dealloc(self.ptr as *mut u8, layout) };

            println!("referece cnt is 0 ans heap value is dropped")
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { &*self.ptr };

        &inner.value
    }
}
