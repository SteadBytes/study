//! Use `Deref` to define a type, `MyBox<T>`, that behaves *like* `Box<T>`.
//! **Note**: `MyBox<T>` does *not* store it's data on the heap! This example
//! is intended to demonstrate the point-like behaviour of `Box<T>` and not how
//! it manages data.

use std::ops::Deref;

fn main() {
    let x = 5; // Holds an i32
    let y = &x; // Holds a reference to x

    assert_eq!(5, x);
    assert_eq!(5, *y); // Dereference y to access x

    // Won't compile - this is asserting that the i32 value 5 is equal to a
    // reference to the variable x
    // assert_eq!(5, y);

    // Using a box like a reference
    let x = 5; // Holds an i32
    let y = Box::new(x); // Holds a box instance pointing to the *value* in x

    assert_eq!(5, x);
    assert_eq!(5, *y); // Dereference on a box follows it's pointer to the value

    // Custom smart pointer - MyBox
    let x = 5; // Holds an i32
    let y = MyBox::new(x); // Holds a box instance pointing to the *value* in x

    assert_eq!(5, x);
    assert_eq!(5, *y); // Rust actually executs *(y.deref())

    // Deref coercion
    let hello = |name: &str| {
        println!("Hello, {}!", name);
    };

    // Pass a string slice directly
    hello("world");

    // Pass MyBox<String> - the deref trait on MyBox<T> allows the compiler to
    // convert &MyBox<String> into &String by calling deref and then &String into
    // &str slice by calling deref on that (standard lib implements Deref for
    // String).
    let m = MyBox::new(String::from("world"));
    hello(&m);

    // Above is equivalent to:
    let m = MyBox::new(String::from("world"));
    // Derefence MyBox<String> to String, & [..] take a str slice to match the
    // signature of hello.
    hello(&(*m)[..]);
}

// Tuple struct with one element of type T
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // Associated type

    fn deref(&self) -> &T {
        // Return reference to the value within the tuple struct.
        // `deref` returns a reference to a value instead of the value directly
        // to ensure the value is owned by self. Returning the value directly
        // would move the value out of self to be owned inside MyBox<T>.
        &self.0
    }
}
