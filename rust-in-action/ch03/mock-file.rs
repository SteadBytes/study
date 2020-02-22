
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>, // Vec for simulating writing to a file (dynamic sizing etc)
}

fn main() {
    let mut f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(), // Simulate empty file
    };

    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.name, &f1.data.len());
}
