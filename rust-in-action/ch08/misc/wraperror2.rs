//! # Simplifying `Error` type usage by implementing `std::convert::From`
//!
//! Demonstration of using `std::convert::From` to remove the need for
//! 'manually' converting `Error` types into a wrapper type (e.g. `map_err`).
//! This provides a cleaner and easier to use API. See `wraperror.rs` for the
//! original version using `map_err`.

use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::net;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Re-use `Debug` for brevity, in a non-trivial example this would
        // likely have  a 'prettier' implementation
        write!(f, "{:?}", self)
    }
}

// Use the default method implementations provided by `error::Error`
impl error::Error for UpstreamError {}

impl From<io::Error> for UpstreamError {
    fn from(error: io::Error) -> UpstreamError {
        UpstreamError::IO(error)
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(error: net::AddrParseError) -> UpstreamError {
        UpstreamError::Parsing(error)
    }
}

fn main() -> Result<(), UpstreamError> {
    // No `map_err`...

    // stdio::io::Error -> UpstreamError::IO
    let _f = File::open("invisible.txt")?;
    // std::net::AddrParseError -> UpstreamError::Parsing
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
