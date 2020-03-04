//! # Deconstructing a Floating Point Value with Bit Manipulation Techniques
//! `f32` is represented using IEE 754-2008 standard `binary32`. This program
//! demonstrates this representation by extracting the components used to
//! store an `f32` value. Made up of 3 components (bit offsets in parens):
//!
//! - Sign bit (31)
//!     - Indicates whether the decimal value is positive or negative
//!       - 1 -> negative
//!       - 0 -> positive
//! - Exponent (30 - 23)
//!     - Special cases:
//!       - `0x00` -> "subnormal number"
//!       - `0xFF` -> infinity, -infinity or NaN
//! - Mantissa (22 - 0)
//!     - Each bit represents a known value defined by the standard
//!     - Special cases:
//!         - `255` -> mantissa == `0` = whole number is infinity, anything
//!         else = whole number is NaN
//!         - `0` -> mantissa == `0` = whole number is 0, every non-zero bit
//!         pattern in mantissa swithces implici 24th bit to 0.0
//!
//! `binary32` bit pattern to decimal:
//!  - `n = -1^sign_bit * radix ^ (exponent - bias) * mantissa`
const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

// TODO: Doc comment for each function & explain `binary32` layout (diagram?)

fn main() {
    let n: f32 = 42.42;

    let (sign_bit, exponent, fraction) = deconstruct_f32(n);
    let (sign, exponent, mantissa) = decode_f32_parts(sign_bit, exponent, fraction);
    let reconstituted_n = f32_from_parts(sign, exponent, mantissa);

    println!(
        "{} -> [sign:{}, exponent:{}, mantissa:{:?}] -> {}",
        n, sign_bit, exponent, mantissa, reconstituted_n
    );
}

fn deconstruct_f32(n: f32) -> (u32, u32, u32) {
    // Cast into unsigned type to allow for bit manipulation
    let n_: u32 = unsafe { std::mem::transmute(n) };

    let sign = (n_ >> 31) & 1; // MSB
    let exponent = (n_ >> 23) & 0xFF; // Strip out MSB and fraction part
    let fraction = 0x7FFFFF & n_; // Least significant 23 bits

    (sign, exponent, fraction)
}

fn decode_f32_parts(sign_bit: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    // Convert sign bit to `1.0` or `-1.0`
    let signed_1 = (-1.0_f32).powf(sign_bit as f32);

    // Treat bits as an integer, then subtract the bias
    let exponent = RADIX.powf(
        ((exponent as i32) - BIAS) // `i32` in case of negative subtraction result
        as f32, // `f32` so it can be used for exponentiation
    );

    let mut mantissa: f32 = 1.0; // Assume implicit 24th bit is set
    for i in 0..23_u32 {
        let one_at_bit_i = 1 << i;
        if (one_at_bit_i & fraction) != 0 {
            // Decimal value of the bit at `i`
            mantissa += 2_f32.powf((i as f32) - 23.0);
        }
    }

    (signed_1, exponent, mantissa)
}

fn f32_from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}
