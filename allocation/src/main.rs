use std::{
    alloc::{Layout, alloc, dealloc, handle_alloc_error, realloc},
    f32::consts,
    mem::{align_of, size_of},
    ptr::{self, NonNull},
};
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

    println!(
        "size : {}, alignment : {}",
        size_of::<Packed>(),
        align_of::<Packed>()
    );

    let p = Packed { a: 32, b: 89 };

    // let x = &p.b; -> unaligned error

    unsafe {
        // let x = ptr::read_unaligned(&p.b);
    }

    learn_layout();
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

struct RawVec<T> {
    ptr: NonNull<T>,
    capacity: usize,
}

impl<T> RawVec<T> {
    fn new(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).unwrap();

        unsafe {
            let ptr = alloc(layout) as *mut T;

            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            Self {
                ptr: unsafe { NonNull::new_unchecked(ptr) },
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

        let new_layout = Layout::array::<T>(cap).unwrap();

        unsafe {
            let new_ptr = if self.capacity == 0 {
                alloc(new_layout)
            } else {
                let old_layout = Layout::array::<T>(self.capacity).unwrap();

                realloc(self.ptr.as_ptr() as *mut u8, new_layout, new_layout.size())
            };

            if new_ptr.is_null() {
                handle_alloc_error(new_layout);
            }
            self.ptr = NonNull::new_unchecked(new_ptr as *mut T);
            self.capacity = cap;
        }
    }
}

fn impl_vec() {}

struct S {
    a: u8,
    b: u64,
}

#[repr(packed)]
struct Packed {
    a: u8,
    b: u64,
}
