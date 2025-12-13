// building arena allocator

use std::{
    alloc::{Layout, LayoutErr, alloc, dealloc, handle_alloc_error},
    ptr::{self, NonNull},
};

type Drop_fn = unsafe fn(*mut u8);

struct DropEntry {
    ptr: *mut u8,     // where the value lives
    drop_fn: Drop_fn, // how to drop it
}

unsafe fn drop_impl<T>(ptr: *mut u8) {
    ptr::drop_in_place(ptr as *mut T);
}

struct Arena {
    buf: NonNull<u8>,
    cap: usize,
    offset: usize,
    drops: Vec<DropEntry>, // drop stack
}

impl Arena {
    fn new(cap: usize) -> Self {
        let layout = Layout::from_size_align(cap, align_of::<usize>()).unwrap();
        // The allocator only understands size + alignment
        // 8 is a safe base alignment for most platforms
        unsafe {
            let ptr = alloc(layout);

            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            Arena {
                buf: NonNull::new_unchecked(ptr),
                cap: cap,
                offset: 0,
                drops: Vec::new(),
            }
        }
    }

    fn align_up(addr: usize, align: usize) -> usize {
        debug_assert!(align.is_power_of_two());
        return (addr + align - 1) & !(align - 1);
    }

    fn alloc_layout(&mut self, layout: Layout) -> NonNull<u8> {
        let start = Self::align_up(self.offset, layout.align());
        let end = start + layout.size();

        if end > self.cap {
            panic!("arena out of bound");
        }

        unsafe {
            let ptr = self.buf.as_ptr().add(start);
            self.offset = end;

            return NonNull::new_unchecked(ptr);
        }
    }

    fn alloc<T>(&mut self, val: T) -> &mut T {
        let layout = Layout::new::<T>();

        let ptr = self.alloc_layout(layout).as_ptr() as *mut T;
        unsafe {
            ptr.write(val);

            self.drops.push(DropEntry {
                ptr: ptr as *mut u8,
                drop_fn: drop_impl::<T>,
            });

            return &mut *ptr;
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        for entry in self.drops.iter().rev() {
            unsafe { (entry.drop_fn)(entry.ptr) };
        }

        let layout = Layout::from_size_align(self.cap, 8).unwrap();

        unsafe {
            dealloc(self.buf.as_ptr(), layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_basic_types() {
        let mut arena = Arena::new(1024);

        let a = arena.alloc(100);
        // let b = arena.alloc(200);

        assert_eq!(*a, 100);
        // assert_eq!(*b, 200);
    }
    #[test]
    fn mixed_alignment_stress() {
        let mut arena = Arena::new(1024);

        {
            let a = arena.alloc(1u8);
            assert_eq!(*a, 1);
        }
        {
            let b = arena.alloc(0x11223344u32);
            assert_eq!(*b, 0x11223344);
        }
        {
            let c = arena.alloc(0x1122334455667788u64);
            assert_eq!(*c, 0x1122334455667788);
        }
    }

    #[test]
    fn destructor_called_exactly_once() {
        use std::cell::Cell;

        struct DropCounter<'a> {
            c: &'a Cell<u32>,
        }

        impl<'a> Drop for DropCounter<'a> {
            fn drop(&mut self) {
                self.c.set(self.c.get() + 1);
            }
        }

        let counter = Cell::new(0);

        {
            let mut arena = Arena::new(512);

            {
                arena.alloc(DropCounter { c: &counter });
            }
            {
                arena.alloc(DropCounter { c: &counter });
            }
            {
                arena.alloc(DropCounter { c: &counter });
            }
        }

        assert_eq!(counter.get(), 3);
    }
    #[test]
    fn drop_order_lifo() {
        use std::cell::RefCell;

        struct Recorder<'a> {
            log: &'a RefCell<Vec<&'static str>>,
            name: &'static str,
        }

        impl<'a> Drop for Recorder<'a> {
            fn drop(&mut self) {
                self.log.borrow_mut().push(self.name);
            }
        }

        let log = RefCell::new(Vec::new());

        {
            let mut arena = Arena::new(512);

            {
                arena.alloc(Recorder {
                    log: &log,
                    name: "A",
                });
            }
            {
                arena.alloc(Recorder {
                    log: &log,
                    name: "B",
                });
            }
            {
                arena.alloc(Recorder {
                    log: &log,
                    name: "C",
                });
            }
        }

        assert_eq!(&*log.borrow(), &["C", "B", "A"]);
    }
}
