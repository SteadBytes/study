//! Demonstration of using `Box<T>` to enable recursive types through
//! constructing a lisp-esque cons list.
//!
//! This recursive implementation will not compile as Rust cannot know the size
//! of the struct a compile time.
//! - `error[E0072]: recursive type `List` has infinite size`
//! ```
//! enum List {
//!     Cons(i32, List),
//!     Nil,
//! }
//! ```
//!
//!  Using a box means that only the *pointer* needs to be stored on the stack
//! and is thus known at compile time. The data pointed to by the box pointer
//! is then allocated on the heap at runtime. As such, this *will* compile:
//! ```
//! pub enum List {
//!    Cons(i32, Box<List>),
//!    Nil,
//! }
//! ```
use self::List::{Cons, Nil};

#[derive(Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub fn demo() {
    println!("Box<T> list demo:");

    let list_i32 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list_i32 = {:?}", list_i32);

    let list_str = Cons(
        "Hello",
        Box::new(Cons(", ", Box::new(Cons("world!", Box::new(Nil))))),
    );
    println!("list_str = {:?}", list_str);
}
