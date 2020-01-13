//! Demonstrate the use of `RefCell<T>` and Interior Mutability by implementing
//! a *mock object* for testing a library which tracks a value against a maximum
//! value and sends messages based on how close the the maximum the current
//! value is (i.e. rate limiting, quota of API calls for a user etc.).

/// Provides the mechanism for sending 'messages' e.g. an email, API call,
/// text message, message another application component etc.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

/// Tracks how close a value is to a given maximum value by using `messenger`
/// to send informational messages whenever `set_value` is called.
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

//! This test code won't compile. It is trying to test `LimitTracker` by
//! asserting on the values of messages sent to `MockMessenger`. However, the
//! implementation of `MockMessenger.send` will not compile as attempts to
//! mutate `MockMessenger.sent_messages` from an immutable reference to `self`.
//! ```
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!
//!     struct MockMessenger {
//!         sent_messages: Vec<String>,
//!     }
//!
//!     impl MockMessenger {
//!         fn new() -> MockMessenger {
//!             MockMessenger {
//!                 sent_messages: vec![],
//!             }
//!         }
//!     }
//!
//!     impl Messenger for MockMessenger {
//!         fn send(&self, message: &str) { // Immutable reference to self
//!             // Attempt to mutate immutable reference
//!             self.sent_messages.push(String::from(message));
//!         }
//!     }
//!
//!     #[test]
//!     fn it_sends_an_over_75_percent_warning_message() {
//!         let mock_messenger = MockMessenger::new();
//!         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
//!
//!         limit_tracker.set_value(80);
//!
//!         assert_eq!(mock_messenger.sent_messages.len(), 1);
//!     }
//! }
//! ```

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // RefCell allows self.sent_messages to be mutably borrowed even
            // though self is an immutable reference
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    // This would compile, but cause a *panic* at runtime as two mutable borrows
    // exist at the same time - `RefCell<T>` does not allow this.
    // ```
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         let mut one_borrow = self.sent_messages.borrow_mut();
    //         let mut two_borrow = self.sent_messages.borrow_mut();
    // 
    //         one_borrow.push(String::from(message));
    //         two_borrow.push(String::from(message));
    //     }
    // }
    // ```

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
