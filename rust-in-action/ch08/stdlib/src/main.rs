//! # Using the Rust standard library to construct an HTTP GET request using
//!
//! Demonstration of directly using TCP via `std::net::TcpStream` to perform
//! simple HTTP requests. This drops one 'layer' down from the previous
//! `simple` example using the `reqwest` library. The low-level details of
//! TCP are abstracted away, however the details of HTTP are not.
use std::{io, io::prelude::*, net::TcpStream};

fn main() -> io::Result<()> {
    // Port must be specified as TcpStream does not know that this will be an
    // HTTP request (port 80)
    let mut connection = TcpStream::connect("www.rustinaction.com:80")?;

    // Request line
    connection.write_all(b"GET / HTTP/1.0")?;
    connection.write_all(b"\r\n")?;

    // Headers
    // Hostname specified previously is discarded after IP address resolution.
    // `Host` header allows the server to know which host is being connected to.
    connection.write_all(b"Host: www.rustinaction.com")?;

    // End HTTP message (two blank lines)
    connection.write_all(b"\r\n\r\n")?;

    // Stream bytes to stdout
    io::copy(&mut connection, &mut io::stdout())?;

    Ok(())
}
