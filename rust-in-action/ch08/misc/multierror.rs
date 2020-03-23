//! # A function that attempts to return multiple `Result` types
//!
//! **Will not compile**
//!
//! Demonstration of compiler error when returning multiple different
//! `Result` types when the function signature specifies only one.
use std::fs::File;
use std::net::Ipv6Addr;

fn main() -> Result<(), std::io::Error> {
    // stdio::io::Error
    let _f = File::open("invisible.txt")?;
    // std::net::AddrParseError
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
