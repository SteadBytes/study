//! Demonstration of using `Rc<T>` to enable shared ownership of items in the
//! cons list.
//!
//! The following won't compile because the `Cons` variants *own* the data they
//! hold.
//! ```
//! mod box_list;
//! 
//! use box_list::List::{Cons, Nil};
//!
//! fn main() {
//! let a = Cons(5,
//!     Box::new(Cons(10,
//!         Box::new(Nil))));
//! let b = Cons(3, Box::new(a)); // Takes ownership of a
//! let c = Cons(4, Box::new(a)); // Tries to take ownership of moved a -> error
//! }
//! ```
//! Using `Rc<T>` instead of `Box<T>` allows both `b` and `c` to hold references
//! to `a`.
use self::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
pub enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

pub fn demo() {
    println!("Rc<T> list demo:");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a = {:?}", a);
    println!("a count after creating a = {}", Rc::strong_count(&a));
    // Not deep-copy clones, Rc::clone increments the reference count
    // -> minimal performance hit
    let b = Cons(3, Rc::clone(&a));
    println!("b = {:?}", b);
    println!("a count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("c = {:?}", c);
        println!("a count after creating c = {}", Rc::strong_count(&a));
    }
    println!(
        "a count after c goes out of scope = {}",
        Rc::strong_count(&a)
    );
}
