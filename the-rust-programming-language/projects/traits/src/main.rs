use std::fmt::Display;

pub trait Summary {
    fn summarise_author(&self) -> String;

    // Default implementation
    fn summarise(&self) -> String {
        // Can call other methods in the same trait
        format!("(Read more from {}...)", self.summarise_author())
    }
}

pub struct Reporter {
    pub name: String,
    pub age: i32,
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: Reporter,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarise_author(&self) -> String {
        format!("{}, {}", self.author.name, self.author.age)
    }

    fn summarise(&self) -> String {
        format!(
            "{}, by {}, ({})",
            self.headline,
            self.summarise_author(),
            self.location
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Use default summarise implementation
impl Summary for Tweet {
    fn summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Trait to constrain function parameter
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarise())
}

// Or with trait bound syntax
// pub fn notify<T: Summary>(item: T) {

// Multiple traits
fn some_function(thing: impl Clone + Ord + Display) {}

// Using a where clause
fn some_other_function<T, U>(t: T, u: U)
where
    T: Display + Ord,
    U: Clone + Ord,
{
}

// Function that returns a type implementing a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("steadbytes"),
        content: String::from("Never laugh at live dragons."),
        reply: false,
        retweet: false,
    }
}

// Note: Cannot use return imple Trait if returning multiple types
// This will not compile
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Smaug wakes from slumber!"),
//             location: String::from("The Lonely Mountain"),
//             author: Reporter {
//                 name: String::from("Gandalf"),
//                 age: 60,
//             },
//             content: String::from("Bilbo woke Smaug and now he's angry."),
//         }
//     } else {
//         Tweet {
//             username: String::from("steadbytes"),
//             content: String::from("Never laugh at live dragons."),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn main() {
    let tweet = Tweet {
        username: String::from("steadbytes"),
        content: String::from("Never laugh at live dragons."),
        reply: false,
        retweet: false,
    };

    // Uses default summarise implementation
    println!("1 new tweet: {}", tweet.summarise());

    let article = NewsArticle {
        headline: String::from("Smaug wakes from slumber!"),
        location: String::from("The Lonely Mountain"),
        author: Reporter {
            name: String::from("Gandalf"),
            age: 60,
        },
        content: String::from("Bilbo woke Smaug and now he's angry."),
    };

    println!("1 new news article: {}", article.summarise());
}
