//! # Creating a `Read` trait
//! Demonstrate the use of a trait by defining a `Read` trait to encompass the
//! ability to read `u8` bytes into buffer `Vec<u8>` e.g. a restricted version
//! of the `std::io::Read` trait for educational purpose - IRL one would use
//! `stdo::io::Read`.
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
    let mut buffer: Vec<u8> = vec![];
    let bytes_read = f6.read(&mut buffer).unwrap();
    f6 = close(f6).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f6);
    println!("{} is {} bytes long", &f6.name, bytes_read);
    println!("{}", text);
}
