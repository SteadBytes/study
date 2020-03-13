//! # `hexdump` clone with hard-coded to mock file I/O
//!
//! First step to a more complete `hexdump` clone.
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;
// Hard-code input for mock file I/O
const INPUT: &'static [u8] = br#"
fn main() {
    println!("Hello, world!")
}"#;

fn main() -> std::io::Result<()> {
    // Mocked file I/O: "read" input into internal buffer
    let mut buffer: Vec<u8> = vec![];
    INPUT.read_to_end(&mut buffer)?;

    for (i, line) in buffer.chunks(BYTES_PER_LINE).enumerate() {
        let byte_offset = i * BYTES_PER_LINE;
        // Up to 8 left-padded 0s
        print!("[0x{:08x}] ", byte_offset);
        for byte in line {
            // 2 digit hex
            print!("{:02x} ", byte);
        }
        println!();
    }

    Ok(())
}
