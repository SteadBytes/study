//! # Attempting to scan through a running program's memory byte by byte,
//! starting at *one* to avoid dereferencing a null pointer.
//!
//! This program **will panic** at runtime due to a segmentation fault.
fn main() {
    let mut n_nonzero = 0;

    // Panics on the first loop due to dereferencing memory that does not
    // belong to this program causing a segmentation fault.
    for i in 1..10000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}
