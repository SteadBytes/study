//! Demonstration of how Rust *does not* guarantee that memory leaks caused by
//! reference cycles cannot be introduced into a program. Rust's memory safety
//! guarantees ensure no *data races* and therefore reference cycles are
//! considered memory safe - though usually undesired!
//!
//! By using `Rc<T>` and `RefCell<T>` references can refer to each other in a
//! cycle. This creates memory leaks because the reference count of each item
//! in the cycle **never reaches 0**, therefore the values will **never be
//! dropped**!
use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

impl<T> List<T> {
    fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn demo() {
    println!("Reference cycle demo:");

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a = {:?}", a);
    println!("a initial rc count = {}", Rc::strong_count(&a));

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("b = {:?}", b);
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b tail = {:?}", b.tail());

    // Create a reference cycle
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // This will overflow the stack!
    // Attemps to follow cycle from a->b->a->b->a->b...
    // Each item in the cycle will never be dropped as it's reference count
    // does not reach 0. Thus each item is allocated memory on the stack until
    // the stack overflows.
    // println!("a next item = {:?}", a.tail());
}
