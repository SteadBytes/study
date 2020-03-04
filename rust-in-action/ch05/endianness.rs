//! # Inspecting Endianness
//! Endianness = the order in which byte sequences are represented.
//! - *Big* endian = MSB -> LSB
//! - *Little* endian = LSB -> MSB
//! Most modern CPUs (e.g. consumer x86) use little endianness.
//! - Note: ARM is bi-endian and can support both but it often used as little
//! endian (e.g. in Android devices)
use std::mem;
fn main() {
    let big_endian: [u8; 4] = [
        0xAA, // 1101_1101
        0xBB, // 1100_1100
        0xCC, // 1011_1011
        0xDD, // 1010_1010
    ];

    let little_endian: [u8; 4] = [
        0xDD, // 1010_1010
        0xCC, // 1011_1011
        0xBB, // 1100_1100
        0xAA, // 1101_1101
    ];

    let (a, b): (i32, i32) = unsafe { (mem::transmute(big_endian), mem::transmute(little_endian)) };

    // Swapped between little vs big endian architecture
    println!("{} vs {}", a, b);
}
