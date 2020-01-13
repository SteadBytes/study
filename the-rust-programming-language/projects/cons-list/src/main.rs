/// Demonstration of using `Box<T>` to enable recursive types through
/// constructing a lisp-esque cons list.
///
/// This recursive implementation will not compile as Rust cannot know the size
/// of the struct a compile time.
/// - `error[E0072]: recursive type `List` has infinite size`
/// ```
/// enum List {
///     Cons(i32, List),
///     Nil,
/// }
/// ```
///
///  Using a box means that only the *pointer* needs to be stored on the stack
/// and is thus known at compile time. The data pointed to by the box pointer
/// is then allocated on the heap at runtime. As such, this *will* compile:
/// ```
/// pub enum List {
///    Cons(i32, Box<List>),
///    Nil,
/// }
/// ```
#[derive(Debug)]
pub enum BoxList<T> {
    Cons(T, Box<BoxList<T>>),
    Nil,
}

/// Demonstration of using `Rc<T>` to enable shared ownership of items in the
/// cons list.
///
/// The following won't compile because the `Cons` variants *own* the data they
/// hold.
/// ```
/// use crate::BoxList::{Cons, Nil};
/// fn main() {
/// let a = Cons(5,
///     Box::new(Cons(10,
///         Box::new(Nil))));
/// let b = Cons(3, Box::new(a)); // Takes ownership of a
/// let c = Cons(4, Box::new(a)); // Tries to take ownership of moved a -> error
/// }
/// ```
/// Using `Rc<T>` instead of `Box<T>` allows both `b` and `c` to hold references
/// to `a` - see example in `main`.
#[derive(Debug)]
pub enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Nil,
}

/// Demonstration of having multiple owners of mutable data by combining `Rc<T>`
/// and `RefCell<T>`. In the `RcList<T>` implementation, the data in the list
/// cannot be mutated as `Rc<T>` holds *only* immutable values. Wrapping a
/// `RefCell<T>` in an `Rc<T>` allows multiple owners and mutation of the value.
/// If multiple owners attempt to mutate the value at the same time, the program
/// will *panic at runtime* - `RefCell<T>` borrows are not checked at compile time.
#[derive(Debug)]
enum RcRefCellList<T> {
    Cons(Rc<RefCell<T>>, Rc<RcRefCellList<T>>),
    Nil,
}

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    {
        println!("BoxList examples:");

        use crate::BoxList::{Cons, Nil};

        let list_i32 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("list_i32 = {:?}", list_i32);

        let list_str = Cons(
            "Hello",
            Box::new(Cons(", ", Box::new(Cons("world!", Box::new(Nil))))),
        );
        println!("list_str = {:?}", list_str);
    }
    {
        println!("\nRcList examples:");

        use crate::RcList::{Cons, Nil};

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
    {
        println!("\nRcRefCellList examples:");

        use crate::RcRefCellList::{Cons, Nil};

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        // This will panic at runtime!
        // let v = value.borrow_mut();
        // let also_v = value.borrow();

        // Multiple immutable borrows are allowed
        let v = value.borrow();
        let also_v = value.borrow();

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}
