#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be in range 1..100, got {}.", value);
        }
        Guess { value }
    }
}

#[cfg(test)] // Mark this module to only be compiled when running tests
mod tests {
    // Bring outer module into scope as the est module is an *inner* module
    // and thus cannot (by default) access the contents of it's parent
    use super::*;

    mod rectangle {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };

            assert!(larger.can_hold(&smaller));
        }

        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };

            assert!(!smaller.can_hold(&larger));
        }
    }

    mod add_two {
        use super::*;

        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }

    mod greeting {
        use super::*;

        #[test]
        fn greeting_contains_name() {
            let result = greeting("Bilbo");
            // Custom failure message
            assert!(
                result.contains("Bilbo"),
                "Greeting did not contain name, value was `{}`",
                result
            );
        }
    }

    mod guess {
        use super::*;

        // Asserting that a panic occurs
        // Similar to pytest.assert_raises()
        #[test]
        // Add custom failure message via 'expected' parameter
        #[should_panic(expected = "Guess value must be in range 1..100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }

    // Using Result<T, E> instead of panicking on failure
    // Allows using '?' operator in tests -> convenient for writing tests that
    // should fail if *any* operation within them returns an Err variant
    // Note: cannot use #[should_panic] with this style of test -> should return
    // an Err value directly when expecting to fail
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }
}
