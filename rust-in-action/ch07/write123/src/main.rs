//! # Writing numbers to disk in a guaranteed byte order
//!
//! Demonstrate the usage of the `byteorder` crate to ensure multi-byte binary
//! data is written and read in a guaranteed byte order. Computing platforms
//! differ in byte order (big vs little endian), programs must account for this
//! in order to work reliably on different platforms.
//!
//! `write123` writes several numbers of different types to disk (actually an
//! in memory `Cursor` wrapping `Vec` as we don't actually need to persist
//! anything), outputting their on disk representation as it does so. It then
//! reads the same numbers from a file (`Cursor`/`Vec`) with the same binary
//! contents and ensures the final values are equal.
extern crate byteorder;

use byteorder::LittleEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut w = vec![];

    let one: u32 = 1;
    let two: i8 = 2;
    let three: f64 = 3.0;

    w.write_u32::<LittleEndian>(one).unwrap();
    println!("{:?}", &w);

    w.write_i8(two).unwrap();
    println!("{:?}", &w);

    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", &w);

    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
    let one_ = r.read_u32::<LittleEndian>().unwrap();
    let two_ = r.read_i8().unwrap();
    let three_ = r.read_f64::<LittleEndian>().unwrap();

    (one_, two_, three_)
}

fn main() {
    let (one, two, three) = write_numbers_to_file();
    let (one_, two_, three_) = read_numbers_from_file();

    assert_eq!(one, one_);
    assert_eq!(two, two_);
    assert_eq!(three, three_);
}
