use std::{
    cell::{Cell, RefCell, UnsafeCell},
    ops::Deref,
};

fn main() {
    // let a = Node {
    //     val: Cell::new(1),
    //     adjacent: vec![],
    //     name: RefCell::new("one".to_string()),
    // };
    // let b = Node {
    //     val: Cell::new(2),
    //     adjacent: vec![&a],
    //     name: RefCell::new("two".to_string()),
    // };
    // let c = Node {
    //     val: Cell::new(3),
    //     adjacent: vec![&a],
    //     name: RefCell::new("three".to_string()),
    // };

    // b.add_urgency();
    // c.add_urgency();cd
    // dbg!(&b);
    // // c.addOne();
    // dbg!(&c);

    // let a = RefCell::new(10);

    // let mut b = a.borrow_mut();
    // let mut c = a.borrow_mut();

    // *b += 1;
    // // *c += 1;
    // dbg!(&b);
    // drop(b);

    // dbg!(c);
}

// now we will se impl of RC

struct Rc<T> {}

// now we will see the impl of the refcell

#[derive(PartialEq, Clone, Copy)]
enum refstate {
    shared(usize),
    unshared,
    exclusive,
}

struct MyRef<T> {
    value: UnsafeCell<T>,
    state: Cell<refstate>,
}

impl<T> MyRef<T> {
    fn new(val: T) -> Self {
        Self {
            value: UnsafeCell::new(val),
            state: Cell::new(refstate::unshared),
        }
    }
    fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            refstate::unshared => {
                self.state.set(refstate::shared(1));
                // return Some(unsafe { &*self.value.get() });

                return Some(Ref { refcell: &self });
            }
            refstate::shared(size) => {
                self.state.set(refstate::shared(size + 1));
                return Some(Ref { refcell: self });
            }
            _ => None,
        }
    }
    fn borrow_mut(&self) -> Option<&mut T> {
        if let refstate::unshared = self.state.get() {
            self.state.set(refstate::exclusive);
            return Some(unsafe { &mut *self.value.get() });
        }
        None
    }
}

struct Ref<'a, T> {
    refcell: &'a MyRef<T>,
}
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            refstate::shared(1) => {
                self.refcell.state.set(refstate::unshared);
            }
            refstate::shared(n) => {
                self.refcell.state.set(refstate::shared(n - 1));
            }
            refstate::unshared | refstate::exclusive => unreachable!(),
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

#[derive(Debug)]
struct Node<'a> {
    val: Cell<i32>,
    name: RefCell<String>,

    adjacent: Vec<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn addOne(&self) {
        self.val.set(self.val.get() + 1);
        for adj in self.adjacent.iter() {
            adj.addOne();
        }
    }
    fn add_urgency(&self) {
        let mut mut_name = self.name.borrow_mut();
        mut_name.push_str("!!!!!");
        for adj in self.adjacent.iter() {
            adj.add_urgency();
        }
    }
}
