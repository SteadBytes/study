//! # Incrementing an Integer Past its Range
//! Demonstration of integer overflow by infinite incrementation.
//! 16-bit integer can represent `65535` *unique* numbers, a `u16` can therefore
//! hold values in the range `0..65536`.
//!
//! Integer overflow behaviour in rust depends on compiler options:
//! - Defaults -> panic
//! - `-O` optimisation -> the value will overflow without stopping the
//! program almost certainly leading to an incorrect program.

fn main() {
    let mut i: u16 = 0;

    // Default compiler options -> panic when `i > 65535`
    // `-O` -> infinite loop with incorrect values for `i`
    loop {
        print!("{}..", i);
        i += 1000;
        if i % 10000 == 0 {
            print!("\n")
        }
        // This would cause a 'literal out or range for `u16`' compiler error
        // if i == 65536 {
        //     println!("Too big!");
        // }
    }
}
