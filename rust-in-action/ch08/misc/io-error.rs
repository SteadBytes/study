//! # A Rust program that always* produces an I/O error
//!
//! * = Assuming that `invisible.txt` is not in `cwd`...
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let _f = File::open("invisible.txt")?;

    Ok(())
}
