fn main() {
    let s = String::from("hello world");
    let w = first_word(&s); // String slice
    println!("The first word of '{}' is '{}'", s, w);

    let hello = &s[..5]; // "hello", type &str
    println!("{}", hello);
    let world = &s[6..11]; // "world", type &str
    println!("{}", world);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // [2, 3], type &[i32]

    for i in slice.iter() {
        println!("{}", i);
    }
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
