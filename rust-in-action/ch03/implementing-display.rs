//! # Implementing `Display` for your own types

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>, // Vec for simulating writing to a file (dynamic sizing etc)
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        // Vec<T>.append *moves* elements from source -> leaving source empty
        // clone `save_to` to avoid truncating original
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        // Pre-reserve space to hold data to minimise allocations insterting byte-by-byte
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut f6 = File::new_with_data("f6.txt", vec![104, 101, 108, 108, 111, 33]);

    f6 = open(f6).unwrap();
    println!("{:?}", f6);
    println!("{}", f6);

    f6 = close(f6).unwrap();
    println!("{:?}", f6);
    println!("{}", f6);
}
