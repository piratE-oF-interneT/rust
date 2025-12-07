use std::{
    alloc::Layout,
    ops::{Deref, DerefMut},
    thread::sleep,
};

pub struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    pub fn new(val: T) -> Self {
        let layout = std::alloc::Layout::new::<T>();

        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };

        unsafe {
            std::ptr::write(ptr, val);
        }

        Self { ptr: ptr }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr } // -> &*self.ptr turns the raw pointer into a safe shared reference
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr } // -> &mut *self.ptr turns the raw pointer into a mutable reference
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            // first read the val then drop it

            std::ptr::drop_in_place(self.ptr);

            // second free the memory block

            let layout = std::alloc::Layout::new::<T>();
            std::alloc::dealloc(self.ptr as *mut u8, layout);

            print!("mybox value is dropped");
        };
    }
}

#[test]
fn test_my_box() {
    let mut my_box = MyBox::new(String::from("helloworld"));

    let mut a = &mut *my_box;
    a.push_str("nice");
    // println!("{a}");
    println!("{}", a)
}
