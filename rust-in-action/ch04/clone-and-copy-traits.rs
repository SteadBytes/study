//! # Using `Clone` and `Copy`
//! Ownership issues can sometimes be resolved by simply *copying* values instead
//! of transferring ownership - each owner (function, struct e.t.c) receives its
//! own copy of the value and thus the value only ever has one owner.
//! - `Copy` trait acts *implicitly*, copying values whenever an object would
//! be moved.
//!   - e.g. Primitive types sch as integers
//! - `Clone` trat acts *explicitly*, the `.clone()` method must be called to
//! initiate the copying.
//!   - e.g. `String`
//!
//! Not always suitable due to the runtime impact of copying data!
//! - Hence `Copy` is only implemented by default on primitive types that are
//! cheap to copy.
#[derive(Debug, Clone, Copy)]
struct CubeSat {
    id: u64,
}

#[derive(Debug, Clone, Copy)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a.clone());
    println!("a: {:?}", a_status.clone());

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);
}
