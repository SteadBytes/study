fn main() {
    // Create new empty string to load data into
    let mut empty = String::new();

    // Create with initial data
    let data = "some initial contents";
    let s = data.to_string();
    let also_s = "some initial contents".to_string();
    let also_s_too = String::from("some initial contents");

    // UTF-8 !
    let hello = String::from("😀😁😎");

    // Appending
    let mut s = String::from("foo");
    s.push(' '); // push appends a single character only
    s.push_str("bar");

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Implementation of add for String takes ownership of it's self param
    // https://doc.rust-lang.org/std/string/struct.String.html#impl-Add%3C%26%27_%20str%3E
    let s3 = s1 + &s2;
    // Note: s1 has been *moved* to s3 and can to longer be used from here on

    // format! macro
    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s3 = String::from("baz");

    let s = format!("{} {} {}", s1, s2, s3); // "foo bar baz"

    // Slicing
    let hello = "Здравствуйте";

    // First 4 **bytes** of the string
    let s = &hello[0..4];

    // String slices *must* start and end at valid **character boundaries** in
    // the UTF-8 string. Slicing into part of a character will cause a panic at
    // runtime. For example, here the first byte is **not** a character boundary.
    // Use string slicing with caution!
    // let s = &hello[0..1];

    // Iteration

    // Each individual Unicode scalar value
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Each raw byte
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
}
