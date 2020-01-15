//! Demonstration of having multiple owners of mutable data by combining `Rc<T>`
//! and `RefCell<T>`. In the `RcList<T>` implementation, the data in the list
//! cannot be mutated as `Rc<T>` holds *only* immutable values. Wrapping a
//! `RefCell<T>` in an `Rc<T>` allows multiple owners and mutation of the value.
//! If multiple owners attempt to mutate the value at the same time, the program
//! will *panic at runtime* - `RefCell<T>` borrows are not checked at compile time.
use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

pub fn demo() {
    println!("RcRefCellList demo:");

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    // This will panic at runtime!
    // let v = value.borrow_mut();
    // let also_v = value.borrow();

    // Multiple immutable borrows are allowed
    let _v = value.borrow();
    let _also_v = value.borrow();

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
