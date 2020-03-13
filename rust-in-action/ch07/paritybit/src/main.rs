//! # Implementing parity bit checking
//!
//! Parity bit checking is a *simple* checksum scheme. An additional bit is
//! stored which indicates whether the number of 1s in a bitstream is even or
//! odd (it's *parity*). If the parity of a bitstream differs from the
//! expected parity indicated by the parity bit then the data has been
//! corrupted.
//!
//! This method is very simple to implement and fast to run, however
//! it has poor reliability - only an **odd number of bit errors** are
//! guarnateed to be detected. If an even number of bits have errors, the
//! correct number of ones will be recorded by the parity bit despite the data
//! being corrupt:
//! ```text
//! data: 1001
//! parity: 0
//!
//! write to disk: 10010
//!
//! corrupted read: 11011
//! parity: 0
//! no error detected
//! ```
//!
//! Traditionally used for error detection in noisy comms systems. ASCII text
//! encoding can conveniently be checked with parity bit checking as it only
//! requires 8 bits of storage (leaving one free for the parity bit).

/// Calculate the parity bit for bitstream `bytes`.
///
/// Returns a `u8` instead of a `bool` so the result can easily be bit shifted
/// into a desired position in a sequence of bits later on.
fn parity_bit(bytes: &[u8]) -> u8 {
    let mut n_ones: u32 = 0;

    for byte in bytes {
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (0b{:08b}) has {} one bits", byte, byte, ones);
    }

    (n_ones % 2 == 0) as u8
}

fn main() {
    let abc = b"abc";
    let parity_abc = parity_bit(abc);
    println!("input: {:?}", abc);
    println!("parity: {:08X}", parity_abc);
    println!();
    let abcd = b"abcd";
    let parity_abcd = parity_bit(abcd);
    println!("input: {:?}", abcd);
    println!("parity: {:08X}", parity_abcd);

    assert_ne!(parity_abc, parity_abcd);
}
