//! # Impossible Addition
//! Integer overflow can also be triggered by mathematical operations which
//! produce results that cannot fit into the target data type.

fn main() {
    let (a, b) = (200, 200);
    // `u8` can hold values in range `0..255`
    // `400 > 255` -> overflow
    // Default compiler options -> panic
    // `-O` -> incorrect result of `144`
    let c: u8 = a + b;
    println!("200 + 200 = {}", c);
}
