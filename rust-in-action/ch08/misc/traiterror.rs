//! # Using a trait object in a return value to simplify error handling
//!
//!
//! Demonstration of using a `Box<dyn Error>` in a `Result` return value to
//! satisfy the compiler when a function returns *multiple* different `Result`
//! types.
//!
//! Using the `Error` trait object allows for runtime polymorphism over any
//! value that implements the `Error` trait. It must be wrapped in a `Box` to
//! enable the size of the return value on the stack to be known at *compile
//! time*.
//! - Concrete `Error` unknown at compile time -> size on the stack unknown
//!
//! However, this method means that the compiler is no longer able to tell that
//! an error has originated upstream - all errors are converted to exactly the
//! sam type.
//! - Trait objects perform **type erasure** - the concrete type is "lost"
//!
//! Furthermore, using a trait object will incur a runtime cost due to the
//! runtime polymorphism requiring additional 'bookkeeping' (vtable etc).

use std::error::Error;
use std::fs::File;
use std::net::Ipv6Addr;

fn main() -> Result<(), Box<dyn Error>> {
    // stdio::io::Error
    let _f = File::open("invisible.txt")?;
    // std::net::AddrParseError
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
