//! # Same bit pattern, different types
//! The underlying bit representation of values are given *meaning* by the
//! type system - the same bit pattern can therefore be used to represent
//! multiple different values.
fn main() {
    // Two *different* numbers represented by the same bit pattern
    // The types determine how the bit pattern is interpreted into a value
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}
