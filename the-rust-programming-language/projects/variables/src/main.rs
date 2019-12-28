fn main() {
    // mutable variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x *= 2;
    println!("The value of x is: {}", x);

    // shadowing
    let y = 5; // note: no mut
    println!("The value of y is: {}", y);
    let y = y * 2;
    println!("The value of y is: {}", y);

    // floating-point types
    let x = 2.0; // f64 (default)

    let y: f32 = 3.0; // f32

    // character type
    // 4 bytes represent a *Unicode Scalar Value*
    let c = 'z'; // note: single quotes
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // compound types
    // tuple
    // fixed length, heterogeneous element types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // access via index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array
    // fixed length, homogeneous element types
    let a = [1, 2, 3, 4, 5]; // inferred type and length
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // explicit type and length specification
    let five_threes = [3; 5]; // 5 elements, each set to value of 3

    // index access
    let first = a[0];
    let second = a[1];
}
