// Semicolon instead of block = load from another file with the same name
mod back_of_house;
mod front_of_house;

// Re-export to allow external code to access hosting module
pub use crate::front_of_house::hosting;

fn serve_order() {}

pub fn eat_at_restaurant() {
    // Absolute path
    // front_of_house is defined in the same module as this fn
    // and can therefore be accessed even though it's private
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Relative path given the 'use' statement above
    hosting::add_to_waitlist();

    // Order a breakfast meal
    let mut meal = back_of_house::Breakfast::summer("Granary");
    // Change our mind about which bread to order after a friend chose it
    meal.toast = String::from("Sourdough");
    println!("I'd like {} toast please", meal.toast);

    // This won't compile if uncommented - meal.seasonal_fruit is private
    // meal.seasonal_fruit = String::from("pear");

    // All members of Appetizer are public -> caller can directly access
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
