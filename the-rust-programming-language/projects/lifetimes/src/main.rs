fn main() {
    // Dangling pointers

    // Won't compile - y does not 'live long enough'
    // {
    //     let x;
    //     {
    //         let y = 10;
    //         y = &x; // y borrows x
    //     }
    //     // y dropped here

    //     println!("x: {}", x); // x refers to borrowed y which does not exist her
    // }

    // Will compile
    {
        let x = 5;
        let y = &x;
        println!("x: {}", x); // y is still 'alive' here
    }

    // Generic lifetimes

    let string1 = String::from("foo");
    let string2 = "bar";
    // str slice and string literal to indicate it should work for both
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Won't compile
    // let string1 = String::from("foo");
    // let result;
    // {
    //     let string2 = "bar";
    //     let result = longest(string1.as_str(), string2); // string2 borrowed here
    // } // string2 dropped here
    // println!("The longest string is {}", result); // string2 needed here

    // Static lifetimes
    // Indicate that a reference *can* live for the entire program duration
    let s: &'static str = "I have a static lifetime.";
}

// Won't compile
// Do not know which of x and y will be returned, therefore the compiler cannot
// determine whether it will be returning a valid reference or not.
// Fix using generic lifetime annotations (below).
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// Lifetime annotation indicates to the compiler that *all* the references in
// the parameters *and* return value *must* have the same lifetime (all at least
// as long as that of the shortest lifetime)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
