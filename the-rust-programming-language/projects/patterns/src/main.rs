fn main() {
    let demo_fns = [
        matching_literals,
        matching_named_variables,
        multiple_patterns,
        matching_ranges,
        destructuring_structs,
        destructuring_enums,
        destructuring_nested_structs_and_enums,
        destructuring_structs_and_tuples,
        ignoring_values,
        match_guards,
        at_bindings,
    ];

    for f in demo_fns.iter() {
        f();
        println!();
    }
}

fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/// Named variables are *irrefutable* patterns. Named variables *within* a `match`
/// block will *shadow* those with the same name outside of the `match` as a new
/// scope is created
fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Shadow the outer y variable - here it's bound to x
        Some(y) => println!("Matched, y = {:?}", y),
        // Outer x variable
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

/// `match` expressions can match multiple patterns usin `|` syntax.
fn multiple_patterns() {
    let x = 1;
    // First arm
    // let x = 2;
    // Second arm
    // let x = 3;
    // Third arm
    // let x = 4;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/// `..=` syntax allows matching an *inclusive* range of values
fn matching_ranges() {
    let x = 5;
    // First arm
    // let x = 1
    // Second arm
    // let x = 6

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    // First arm
    let x = 'f';
    // Second arm
    let x = 'o';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_structs() {
    let p = Point { x: 0, y: 7 };

    // Explicit named destructuring
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Shorthand for using the same variable names as in the struct
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Destructuring with literal values to test the value of some fields
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

/// Pattern used to destructure an enum should correspond to the way the data
/// stored within the enum is defined.
fn destructuring_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    // Each arm uses a pattern which corresponds the data structure in the enum
    // variant that it matches
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change the color to RGB({}, {}, {})", r, g, b),
    }
}

fn destructuring_nested_structs_and_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    // Destructure the nested `Color` enum in the `ChangeColor` variant to
    // determine which color representation (RGB or HSV) is being used
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to RGB({}, {}, {})", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to HSV({}, {}, {})", h, s, v)
        }
        _ => (), // Other matches would be the same as in `destructuring_enums()`
    }
}

/// Destructuring patterns can be mixed, matched and nested as needed to access
/// values in more complex ways.
fn destructuring_structs_and_tuples() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!(
        "The wall at location x = {}, y = {} is {} feet {} inches high",
        x, y, feet, inches
    );
}

/// `_` ignores entire values or parts of values, `..` ignores *remaining* parts
/// of a value.
fn ignoring_values() {
    fn ignore_an_entire_value(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    ignore_an_entire_value(3, 4);

    // Ignore parts of a value with nested `_`
    // Here, a user cannot *overwrite* an existing setting customisation, but
    // can *unset* it and give it a value if currently unset.
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customised value");
        }
        // This allows setting to None no matter what the current value of
        // the setting OR setting to Some if the current value is None
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // Ignore particular values within a pattern
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }

    // Ignoring an unused variable by prefixing with `_`. Unused variables will
    // generate compiler warnings as it could be a bug
    let _x = 5;
    // This would generate a warning as x is not used
    // let x = 5;
    let y = 10;
    println!("y = {}", y);

    // `_<name>` binds a value, `_` does not
    let s = Some(String::from("Hello!"));

    // This won't compile!
    // `s` is moved into `_s` in the match arm event though it isn't used. This
    // means that the `println!` will fail as `s` has been moved
    // if let Some(_s) = s {
    //     println!("found a string");
    // }

    // This is ok because `_` will not move `s`.
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    // Ignoring remaining parts of a value with `..`

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // `..` will *expand* to as many values as it *needs* to be
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // This won't compile!
    // `..` *must* be unambiguous - i.e. it must be clear which values should be
    // matched and which values should be ignored.
    // match numbers {
    //     (.., second, ..) => println!("Some numbers: {}", second),
    // }
}

/// A match guard is an additional `if` condiation specified *after* a `match`
/// arm patter that **must also match** for that arm to be chosen. Used to
/// express more complex conditions that a pattern alone is able to.
fn match_guards() {
    let num = Some(4);

    match num {
        // x < 5 cannot be expressed with a pattern alone
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // Use match guard to avoid shadowing outer variables in order to test a
    // value against a variable outside of a `match` block
    // This is because match guards **are not patterns* and thus don't bind new
    // variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    // Combine with `|` to specify multiple patterns
    let x = 4;
    let y = false;

    match x {
        // `if y` guard applies to *all* of the values 4, 5 and 6
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

/// `@` operator allows creating variables that hold a value at the same time
/// as testing the value to see whether it matches a pattern.
fn at_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // Captures the value that matched within the range as `id_variable`
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
