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
//! - Significand (22 - 0)
//!     - Each bit represents a known value defined by the standard
//!     - Special cases:
//!         - `255` -> significand == `0` = whole number is infinity, anything
//!         else = whole number is NaN
//!         - `0` -> significand == `0` = whole number is 0, every non-zero bit
//!         pattern in significand swithces implici 24th bit to 0.0
//!
//! `binary32` bit pattern to decimal:
//!  - `n = -1^sign_bit * radix ^ (exponent - bias) * significand`
const BIAS_F32: i32 = 127;
const RADIX_F32: f32 = 2.0;

const BIAS_F64: i64 = 1023;
const RADIX_F64: f64 = 2.0;

// TODO: Doc comment for each function & explain `binary32` layout (diagram?)

fn main() {
    println!("binary32:");
    let n: f32 = 42.42;

    let encoded = Binary32EncodedParts::new(n);
    println!(
        "{} -> [sign:{:01b}, exponent:{:08b}, significand:{:023b}] -> tbc",
        n, encoded.sign, encoded.exponent, encoded.fraction
    );

    let decoded = Binary32DecodedParts::new(&encoded);
    let reconstituted_n = decoded.to_float();

    println!(
        "{} -> [sign:{}, exponent:{}, significand:{}] -> {}",
        n, decoded.sign, decoded.exponent, decoded.significand, reconstituted_n
    );
    println!("binary64:");
    let n: f64 = 42.42;

    let encoded = Binary64EncodedParts::new(n);
    println!(
        "{} -> [sign:{:01b}, exponent:{:08b}, significand:{:023b}] -> tbc",
        n, encoded.sign, encoded.exponent, encoded.fraction
    );

    let decoded = Binary64DecodedParts::new(&encoded);
    let reconstituted_n = decoded.to_float();

    println!(
        "{} -> [sign:{}, exponent:{}, significand:{}] -> {}",
        n, decoded.sign, decoded.exponent, decoded.significand, reconstituted_n
    );
}

// TODO: Remove Parts impls -> define a trait that returns the Structs directly?
// TODO: Reduce duplication

#[derive(Debug, PartialEq)]
struct Binary32EncodedParts {
    sign: u32,
    exponent: u32,
    fraction: u32,
}

impl Binary32EncodedParts {
    fn new(n: f32) -> Binary32EncodedParts {
        // Cast into unsigned type to allow for bit manipulation
        // Use `unsafe {std::mem::transmute(n)};` on Rust < 1.20
        let bits = n.to_bits();

        let sign = (bits >> 31) & 1; // MSB
        let exponent = (bits >> 23) & 0xFF; // Strip out MSB and fraction part
        let fraction = 0x7FFFFF & bits; // Least significant 23 bits

        Binary32EncodedParts {
            sign,
            exponent,
            fraction,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Binary32DecodedParts {
    sign: f32,
    exponent: f32,
    significand: f32,
}

impl Binary32DecodedParts {
    fn new(encoded: &Binary32EncodedParts) -> Binary32DecodedParts {
        // Convert sign bit to `1.0` or `-1.0`
        let signed_1 = (-1.0_f32).powf(encoded.sign as f32);

        // Treat bits as an integer, then subtract the bias
        let exponent = RADIX_F32.powf(
            ((encoded.exponent as i32) - BIAS_F32) // `i32` in case of negative subtraction result
        as f32, // `f32` so it can be used for exponentiation
        );

        let mut significand: f32 = 1.0; // Assume implicit 24th bit is set
        for i in 0..23_u32 {
            let one_at_bit_i = 1 << i;
            if (one_at_bit_i & encoded.fraction) != 0 {
                // Decimal value of the bit at `i`
                significand += 2_f32.powf((i as f32) - 23.0);
            }
        }

        Binary32DecodedParts {
            sign: signed_1,
            exponent,
            significand,
        }
    }

    fn to_float(&self) -> f32 {
        self.sign * self.exponent * self.significand
    }
}

#[derive(Debug, PartialEq)]
struct Binary64EncodedParts {
    sign: u64,
    exponent: u64,
    fraction: u64,
}

impl Binary64EncodedParts {
    fn new(n: f64) -> Binary64EncodedParts {
        // Cast into unsigned type to allow for bit manipulation
        // Use `unsafe {std::mem::transmute(n)};` on Rust < 1.20
        let bits = n.to_bits();

        let sign = (bits >> 63) & 1; // MSB
        let exponent = (bits >> 52) & 0x7FF; // Strip out MSB and fraction part
        let fraction = 0xFFFFFFFFFFFFF & bits; // Least significant 52 bits

        Binary64EncodedParts {
            sign,
            exponent,
            fraction,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Binary64DecodedParts {
    sign: f64,
    exponent: f64,
    significand: f64,
}

impl Binary64DecodedParts {
    fn new(encoded: &Binary64EncodedParts) -> Binary64DecodedParts {
        // Convert sign bit to `1.0` or `-1.0`
        let signed_1 = (-1.0_f64).powf(encoded.sign as f64);

        // Treat bits as an integer, then subtract the bias
        let exponent = RADIX_F64.powf(
            ((encoded.exponent as i64) - BIAS_F64) // `i64` in case of negative subtraction result
        as f64, // `f64` so it can be used for exponentiation
        );

        let mut significand: f64 = 1.0; // Assume implicit 53rd bit is set
        for i in 0..51_u64 {
            let one_at_bit_i = 1 << i;
            if (one_at_bit_i & encoded.fraction) != 0 {
                // Decimal value of the bit at `i`
                significand += 2_f64.powf((i as f64) - 52.0);
            }
        }

        Binary64DecodedParts {
            sign: signed_1,
            exponent,
            significand,
        }
    }

    fn to_float(&self) -> f64 {
        self.sign * self.exponent * self.significand
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! deconstruct_f32_tests {
        ($($name:ident: $n:expr, $sign_bit:expr, $exponent:expr, $fraction:expr,)*) => {
        $(
            #[test]
            fn $name() {

                let expected =  Binary32EncodedParts{
                    sign: $sign_bit,
                    exponent: $exponent,
                    fraction:$fraction
                };

                let parts = Binary32EncodedParts::new($n);
                assert_eq!(parts, expected);
            }
        )*
        }
    }

    deconstruct_f32_tests! {
        deconstruct_1: 1.0, 0, 127, 0,
        deconstruct_2: 0.25, 0, 125, 0,
        deconstruct_3: 0.375, 0, 125, 4194304,

        deconstruct_4: -1.0, 1, 127, 0,
        deconstruct_5: -0.25, 1, 125, 0,
        deconstruct_6: -0.375, 1, 125, 4194304,
    }
}
