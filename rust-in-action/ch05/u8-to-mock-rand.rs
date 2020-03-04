//! # Generating `f32` values between 0 and 1 from random bytes
//! Convert from some source of random bytes into floating point values
//! between `0..1`
use std::fs::File;
use std::io::Read;
use std::iter::Zip;

/// Generates an `f32` that lies between `0..1` from an arbitrary input byte
/// through division. Relatively slow compared to `mock_rand`, but with a
/// slightly larger range (exactly `0..1`).
fn mock_rand_division(n: u8) -> f32 {
    (n as f32) / 255.0 // max value of `u8`
}

/// Generates an `f32` that lies between `0..1` from an arbitrary input byte
/// by using the bits of `n` as the *mantissa* of an `f32` with a constant
/// exponent of -1 (`0b01111110`). Faster than `mock_rand_division` but with a
/// slightly smaller range (~`0..0.996`)
fn mock_rand(n: u8) -> f32 {
    // Underscores mark sign/exponent/mantissa boundaries
    let base: u32 = 0b0_01111110_00000000000000000000000;
    // Align `n` to 32 bits and increase it's value
    let large_n = (n as u32) << 15;
    // Merge into `base` to fill the mantissa
    let f32_bits = base | large_n;
    // Interpret as `f32`
    let m = f32::from_bits(f32_bits);
    // Normalize output range
    2.0 * (m - 0.5)
}

fn main() {
    // Demonstrate ranges of both methods
    println!("{:-^30}", "mock_rand_division");
    println!(
        "max of input range: {:08b} -> {:?}",
        0xff,
        mock_rand_division(0xff)
    );
    println!(
        "mid of input range: {:08b} -> {:?}",
        0x77,
        mock_rand_division(0x77)
    );
    println!(
        "min of input range: {:08b} -> {:?}",
        0x00,
        mock_rand_division(0x00)
    );

    println!("{:-^30}", "mock_rand");
    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {:?}", 0x77, mock_rand(0x77));
    println!("min of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));

    // Demonstrate generating values from random bytes
    println!("{:-^30}", "/dev/urandom -> mock_rand");
    let mut f = File::open("/dev/urandom").unwrap();
    let mut buf = [0u8; 16];
    f.read_exact(&mut buf).unwrap();
    let rands = buf.iter().copied().map(mock_rand);
    for (x, y) in buf.iter().zip(rands) {
        println!("{:08b} -> {:?}", x, y);
    }
}
