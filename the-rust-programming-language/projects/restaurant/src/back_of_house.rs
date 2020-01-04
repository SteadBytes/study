pub struct Breakfast {
    // Allow customer to specify their toast type -> public
    pub toast: String,
    // Chef chooses fruit based on what's in season -> private
    seasonal_fruit: String,
}

impl Breakfast {
    // Private field on Breakfast *requires* that the struct provide a
    // function to create an instance of it. Otherwise, it would be unuseable
    // because the caller cannot set the private field
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("mango"),
        }
    }
}

// All members are public -> no constructor function required
pub enum Appetizer {
    Soup,
    Salad,
}

fn fix_incorrect_order() {
    cook_order();
    // super is 'crate' here, which allows access to the top level
    // serve_order function
    super::serve_order();
}

fn cook_order() {}