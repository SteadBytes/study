use std::io::prelude::*; // access traits for reading/writing streams
use std::net::{TcpListener, TcpStream};

fn main() {
    // Panic if binding fails e.g. insufficient privileges, port already
    //  in use etc
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Iterate over `TcpStream`s representing **conection attempts** from
    // clients
    for stream in listener.incoming() {
        // `TcpStream` automatically closes client connection when dropped
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

/// Handle an incoming TCP connection by printing the contents to the terminal.
/// Note: Suitable only for **small requests** - `< 512` bytes.
///
/// Note: `TcpStream` internally tracks the data read from it to perform
/// buffering -> must be `mut` as it's internal state might change
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    // Print out the contents of the request read into the buffer, replacing
    // invalid UTF-8 sequences with `U+FFD REPLACEMENT CHARACTER`
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
