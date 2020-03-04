//! # Inspecting a Float's Bit String by Interpreting as an Integer
//! `fmt::Binary` is required to pring a value as  individual bits. `f32` does
//! not implement `fmt::Binary` however Integer types *do*. Treating an `f32` as
//! a `u32` **without affecting the underlying bits** will enable printing its
//! bit pattern.
//! - Could also use any other Integer type that is guaranteed to use the same
//! number of bits e.g. `i32`
fn main() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{:032b}", frankentype);
}
