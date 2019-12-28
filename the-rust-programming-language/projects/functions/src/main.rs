fn main() {
    another_function(5, 15);

    // statement
    let x = 5;

    // block expression
    let y = {
        // block *evaluates* to 4
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let y = plus_one(5);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 // implicit return
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
