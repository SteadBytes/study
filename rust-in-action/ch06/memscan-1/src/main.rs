//! # Attempting to scan through a running program's memory byte by byte,
//! starting at zero.
//!
//! This program **will panic** at runtime due to a null pointer exception.
fn main() {
    let mut n_nonzero = 0;

    // Panics on the first loop due to 0 being a null pointer
    for i in 0..10000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}
