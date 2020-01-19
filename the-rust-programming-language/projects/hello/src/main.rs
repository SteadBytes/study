use hello::ThreadPool;
use std::fs;
use std::io::prelude::*; // access traits for reading/writing streams
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    // Panic if binding fails e.g. insufficient privileges, port already
    //  in use etc
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4).unwrap();

    // Iterate over `TcpStream`s representing **conection attempts** from
    // clients
    for stream in listener.incoming() {
        // `TcpStream` automatically closes client connection when dropped
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

/// Handle an incoming TCP connection.
/// - `/` -> `200`, contents of `hello.html`
/// - `/sleep` -> Simulate a slow response with a `5s` sleep, then `200`, contents
/// of `hello.html`
/// - `_` -> `404`
///
/// Note: Up to `512` bytes of the incoming request will be printed to the
/// terminal for logging.
///
/// Note: `TcpStream` internally tracks the data read from it to perform
/// buffering -> must be `mut` as it's internal state might change
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // Print out the contents of the request read into the buffer, replacing
    // invalid UTF-8 sequences with `U+FFD REPLACEMENT CHARACTER`
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // *Rudimentary* path routing and error handling
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // Simulate a slow request
        let d = Duration::from_secs(5);
        println!("Sleeping for {:?} ...", d);
        thread::sleep(d);
        println!("Done");
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
