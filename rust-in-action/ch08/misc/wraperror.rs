//! # Wrapping upstream errors to retain type safety
//!
//! Demonstration of returning multiple error types from a function without
//! the type erasure from using a trait object (see `traiterror.rs`) by
//! wrapping multiple error types in a single type (`Enum` in this case). This
//! has the benefit of providing *explicit*, compile time guarantees about the
//! concrete error types that can be expected. Furthermore, it does not incur
//! the runtime cost associated with trait objects and runtime polymorphism.
//!
//! ## General steps:
//!
//! 1. Define an enum with variants for each error.
//! 2. Annotate the enum with `#[derive(Debug)]`.
//! 3. Implement `Display`.
//! 4. Implment `Error`.
//! 5. One/both of:
//!     a. Use `map_err()` to convert errors into the correct variant of the new
//!        wrapper type.
//!     b. Implement `std::convert::From` for each error type to the wrapper
//!        type to avoid the need for `map_err()` (see `wraperror2.rs`).

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

fn main() -> Result<(), UpstreamError> {
    // stdio::io::Error -> UpstreamError::IO
    let _f = File::open("invisible.txt").map_err(UpstreamError::IO)?;
    // std::net::AddrParseError -> UpstreamError::Parsing
    let _localhost = "::1".parse::<Ipv6Addr>().map_err(UpstreamError::Parsing)?;

    Ok(())
}
