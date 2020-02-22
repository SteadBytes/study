//! # Simplifying object creation by implementing a `new()` method
//! Creating objects with *reasonable defaults* is implementing (by convention)
//! using `new()` method. Variations on `new()` for other defaults are
//! typically defined using `new_some_other_thing()` methods.
//! - Both are similar to constructor/factory methods from other languages
#![allow(unused_variables)] // Relax compiler warnings whilst experimenting

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>, // Vec for simulating writing to a file (dynamic sizing etc)
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(&self, save_to: &mut Vec<u8>) -> usize {
        // Vec<T>.append *moves* elements from source -> leaving source empty
        // clone `save_to` to avoid truncating original
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        // Pre-reserve space to hold data to minimise allocations insterting byte-by-byte
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(f: &mut File) -> bool {
    true // Not a real implementation
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    let mut blank = File::new("blank.txt");

    let mut f3 = File::new_with_data("f3.txt", vec![104, 101, 108, 108, 111, 33]);

    for mut f in [blank, f3].iter_mut() {
        let mut buffer: Vec<u8> = vec![];
        open(&mut f);
        let bytes_read = f.read(&mut buffer);
        close(&mut f);

        // `read` does no conversions - reading raw bytes from a `File`.
        let text = String::from_utf8_lossy(&buffer);

        println!("{:?}", f);
        println!("{} is {} bytes long", &f.name, bytes_read);
        println!("{}", text)
    }
}
