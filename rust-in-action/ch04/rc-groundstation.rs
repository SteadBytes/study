//! # Wrap Data Within Specialty Types
//! Wrapper types can be used to present a 'public' facade that satisfies  move
//! semantics whilst doing something else under the hood.
//! `Rc<T>` (reference counted type `T`) provides runtime garbage collection
//! via reference counting - allowing multiple ownership.
//! `RefCell<T>` allows for *interior mutability* to allow an immutable public
//! facade that performs mutation under the hood.
//!
//! `Rc<T>` and `RefCell<T>` incur additional runtime overhead and are *not
//! thread safe*.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64, // Mhz
}

fn main() {
    let base: Rc<RefCell<GroundStation>> =
        Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));

    println!("base: {:?}", base);

    {
        // introduce a new scope
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
