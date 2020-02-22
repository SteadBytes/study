//! # Modelling Files With Struct
#![allow(unused_variables)] // Relax compiler warnings whilst experimenting

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>, // Vec for simulating writing to a file (dynamic sizing etc)
}

fn open(f: &mut File) -> bool {
    true // Not a real implementation
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    // Vec<T>.append *moves* elements from source -> leaving source empty
    // clone `save_to` to avoid truncating original
    let mut tmp = f.data.clone();
    let read_length = tmp.len();
    // Pre-reserve space to hold data to minimise allocations insterting byte-by-byte
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let mut f2 = File {
        name: String::from("f2.txt"),
        data: vec![104, 101, 108, 108, 111, 33], // Simulate file contents
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let bytes_read = read(&f2, &mut buffer);
    close(&mut f2);

    // `read` does no conversions - reading raw bytes from a `File`.
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, bytes_read);
    println!("{}", text)
}
