//! # Making an HTTP request with the `reqwest` library
//!
//! Demonstration of using a high level library to make a simple HTTP request.
//! `reqwest` serves the same purpose as the popular Python `requests` library.
//!
//! `reqwest` abstracts away many of the details involved in HTTP:
//! - Opening/closing HTTP connections
//! - Converting byte stream to content
//! - TCP port numbers (e.g. HTTP defaults to `80`, HTTPS default to `443`)
//! - IP addresses and DNS resolution
//!
//! The return type of `main` shows the use of a **trait object** for *runtime*
//! polymorphism. `Box<dyn std::error::Error>` equates to "a `Box` (pointer) to
//! *any* type that implements `std::error::Error`". This is used to support the
//! multiple different *concrete* error types that may be encountered within
//! `main`.
extern crate reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://www.rustinaction.com";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    print!("{}", content);

    Ok(())
}
