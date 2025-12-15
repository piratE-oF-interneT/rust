use std::{
    alloc::{Layout, alloc, dealloc, handle_alloc_error},
    marker::PhantomData,
    ptr::{self, NonNull},
};

struct Slab<T> {
    buf: NonNull<u8>,
    free_head: *mut u8,
    capacity: usize,
    used: usize,
    _marker: PhantomData<T>,
}

fn slot_size<T>() -> usize {
    size_of::<T>().max(size_of::<*mut u8>())
}

fn slot_align<T>() -> usize {
    align_of::<T>().max(align_of::<*mut u8>())
}
impl<T> Slab<T> {
    fn new(cap: usize) -> Self {
        let slot = slot_size::<T>();

        let align = slot_align::<T>();

        let size = slot * cap;

        let layout = Layout::from_size_align(size, align).unwrap();

        let ptr = unsafe { alloc(layout) };

        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        unsafe {
            for i in 0..cap {
                let slot_ptr = ptr.add(i * slot);

                let next = if i + 1 < cap {
                    ptr.add((i + 1) * slot)
                } else {
                    ptr::null_mut()
                };

                *(slot_ptr as *mut *mut u8) = next;
            }
        }

        Self {
            buf: unsafe { NonNull::new_unchecked(ptr) },
            capacity: cap,
            free_head: ptr,
            used: 0,
            _marker: PhantomData,
        }
    }

    fn alloc(&mut self, val: T) -> Option<*mut T> {
        if self.free_head.is_null() {
            return None;
        }

        unsafe {
            let slot = self.free_head;
            self.free_head = *(slot as *mut *mut u8);

            (slot as *mut T).write(val);
            self.used += 1;
            Some(slot as *mut T)
        }
    }
    unsafe fn free(&mut self, ptr: *mut T) {
        ptr::drop_in_place(ptr);

        self.used -= 1;

        let slot = ptr as *mut u8;

        *(slot as *mut *mut u8) = self.free_head;
        self.free_head = slot;
    }
}

impl<T> Drop for Slab<T> {
    fn drop(&mut self) {
        let slot = slot_size::<T>();
        let align = slot_align::<T>();

        let size = slot * self.capacity;

        let layout = Layout::from_size_align(size, align).unwrap();

        unsafe {
            dealloc(self.buf.as_ptr(), layout);
        }
    }
}
