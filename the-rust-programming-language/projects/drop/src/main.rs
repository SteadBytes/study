#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let a = CustomSmartPointer {
        data: String::from("Hello, "),
    };
    let b = CustomSmartPointer {
        data: String::from("world!"),
    };
    println!("Created `a = {:?}`, `b = {:?}`", a, b);

    // Dropping a value early
    let c = CustomSmartPointer {
        data: String::from("foo"),
    };
    println!("Created `c = {:?}`", c);
    drop(c);
    println!("Dropped `c` before the end of main");
}
