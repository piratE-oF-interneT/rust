use std::{
    alloc::{Layout, alloc, dealloc, handle_alloc_error, realloc},
    f32::consts,
    mem::{MaybeUninit, align_of, size_of},
    ptr::{self, NonNull, drop_in_place},
};
pub mod arena;
pub mod slab;
fn main() {
    // println!("{}", size_of::<u8>());
    // println!("{}", size_of::<u16>());
    // println!("{}", size_of::<u32>());
    // println!("{}", size_of::<u64>());
    // println!("{}", size_of::<f32>());
    // println!("{}", size_of::<f64>());

    // println!("------------------------------------------------------");

    // println!("{}", align_of::<u8>());
    // println!("{}", align_of::<u16>());
    // println!("{}", align_of::<u32>());
    // println!("{}", align_of::<u64>());
    // println!("{}", align_of::<f32>());
    // println!("{}", align_of::<f64>());

    // println!("size : {}, alignment : {}", size_of::<S>(), align_of::<S>());

    // println!(
    //     "size : {}, alignment : {}",
    //     size_of::<Packed>(),
    //     align_of::<Packed>()
    // );

    let p = Packed { a: 32, b: 89 };

    // let x = &p.b; -> unaligned error

    unsafe {
        // let x = ptr::read_unaligned(&p.b);
    }

    // learn_layout();

    impl_vec();
}

fn learn_layout() {
    // layout creation
    // 1. from type

    use std::alloc::Layout;

    let layout = Layout::new::<u32>(); // size : 4 , alignment : 4

    // 2. from manual size + alignment

    let layout2 = Layout::from_size_align(24, 16).expect("error while creating layout");
    // error occurs when align is not power of two

    // 3. array layout

    let layout_array = Layout::array::<u16>(10).unwrap();

    // size : 1*10 = 20 bytes
    // align : 2

    // let (combined, offset) = layout.extend(layout2).unwrap();

    let (combined, offset) = layout2.extend(layout).unwrap();

    // println!("{}, {} , {}", offset, combined.size(), combined.align());

    // note : extend is internally used by struct for memort allocation of its fields

    //  now after creating layout , allocate heap to layout

    let base_ptr = unsafe { alloc(combined) };

    let header_ptr = base_ptr as *mut u8;

    let payload_ptr = unsafe { base_ptr.add(offset) as *mut u64 };

    unsafe {
        header_ptr.write(1);
        payload_ptr.write(89999);
    }

    unsafe {
        let h = ptr::read(base_ptr as *const u8);
        let p = ptr::read(base_ptr.add(offset) as *const u64);

        println!("header : {} , payload : {}", h, p);
    }

    unsafe {
        dealloc(base_ptr, combined);
    }

    /*  Compute layouts

    Extend layouts

    Allocate once

    Place objects via offsets */
}

use std::fmt::Display;

struct RawVec<T> {
    ptr: NonNull<MaybeUninit<T>>,
    capacity: usize,
}

struct MyVec<T> {
    buf: RawVec<T>,
    len: usize, // current offset
}

impl<T> RawVec<T> {
    fn new(capacity: usize) -> Self {
        let layout = Layout::array::<MaybeUninit<T>>(capacity).unwrap();

        unsafe {
            let ptr = alloc(layout) as *mut MaybeUninit<T>;

            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            println!("raw vec allocated");
            Self {
                ptr: unsafe { NonNull::new_unchecked(ptr as *mut MaybeUninit<T>) },
                capacity: capacity,
            }
        }
    }

    fn grow(&mut self) {
        let cap = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };

        let new_layout = Layout::array::<MaybeUninit<T>>(cap).unwrap();

        unsafe {
            println!("reallocation inittiated for cap : {}", self.capacity);
            let new_ptr = if self.capacity == 0 {
                alloc(new_layout)
            } else {
                let old_layout = Layout::array::<MaybeUninit<T>>(self.capacity).unwrap();

                realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size())
            };

            if new_ptr.is_null() {
                handle_alloc_error(new_layout);
            }
            self.ptr = NonNull::new_unchecked(new_ptr as *mut MaybeUninit<T>);
            self.capacity = cap;
        }
    }
}
impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.capacity == 0 {
            return;
        }

        let layout = Layout::array::<MaybeUninit<T>>(self.capacity).unwrap();

        unsafe {
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

impl<T> MyVec<T> {
    fn new(cap: usize) -> Self {
        Self {
            buf: RawVec::new(cap),
            len: 0,
        }
    }

    fn push(&mut self, val: T) {
        let old_cap = self.buf.capacity;
        if self.len == old_cap {
            // realloc buffer

            self.buf.grow();
        }

        unsafe {
            let dest = self.buf.ptr.as_ptr().add(self.len);

            dest.write(MaybeUninit::new(val));

            self.len += 1;
        };
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        unsafe {
            self.len -= 1;
            Some(self.buf.ptr.as_ptr().add(self.len).read().assume_init())
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                let ptr = self.buf.ptr.as_ptr().add(i).cast::<T>();
                drop_in_place(ptr);
            }
        }
    }
}

fn impl_vec() {
    let mut myvec = MyVec::new(0);
    myvec.push(10);
    myvec.push(20);
    myvec.push(30);
    myvec.push(40);
    myvec.push(50);

    // println!("{}", myvec);
}

struct S {
    a: u8,
    b: u64,
}

#[repr(packed)]
struct Packed {
    a: u8,
    b: u64,
}
