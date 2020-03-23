//! # Example of using a trait object, `&Enchanter` to enable a container to
//! hold several concrete types
//!
//! Demonstration of **runtime polymorphism** using *trait objects*. In this
//! example, multiple values of different concrete types are stored in a `Vec`
//! by using a trait object for a common tratit between those concrete types.
//!
//! Trait objects are commonly used for:
//! - Heterogenous collections of objects
//! - Returning multiple concrete types from a function
//! - Supporting *dynamic dispatch*
//!     - Function to call is determined at runtime instead of compile time
use rand;
use rand::seq::SliceRandom;
use rand::Rng;

// Different concrete character types

#[derive(Debug)]
struct Dwarf {}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}

/// Represents the ability for a character to enchant a `Thing` with some
/// level of competency. A higher competency = higher chance of successful
/// enchantment.
trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    /// Enchant `thing` with success rate determined by `self.competency`. A
    /// failed enchantment turns `thing` into a worthless `thing::Trinket`.
    fn enchant(&self, thing: &mut Thing) {
        print!("{:?} mutters incoherently.", self);
        if rand::thread_rng().gen_bool(self.competency()) {
            println!("The {:?} glows brightly", thing);
            return;
        }

        println!(
            "The {:?} fizzes, then turns into a worthless trinket.",
            thing
        );
        *thing = Thing::Trinket;
    }
}

// Implement a common `Enchanter` trait for each character type

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}

fn main() {
    let mut it = Thing::Sword;

    // Characters
    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};

    // Each character has a different concrete type, however they all implement
    // the `Enchanter` trait. To store them all in a single `Vec`, a *trait*
    // object `&dyn Enchanter` can be used. A compile time ltype error will
    // occur if any of the characters **do not** implement `Enchanter`.
    let party: Vec<&dyn Enchanter> = vec![&d, &h, &e];
    // Randomly choose a character to perform the enchantment
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut it);
}
