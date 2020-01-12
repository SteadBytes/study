use rand::Rng;

use add_one;
use add_two;

fn main() {
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen();
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );

    println!(
        "Hello, world! {} plus two is {}!",
        num,
        add_two::add_two(num)
    );
}
