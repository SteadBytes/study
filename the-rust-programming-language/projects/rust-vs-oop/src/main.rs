//! Demonstration of the various ways in which Rust provides "OOP-like"
//! functionality and how it differs from typical OOP languages.

/// Collection of `i32` values with average (arithmetic mean) caching. The
/// average for a particular state of the collection is calculated only once
/// and not re-computed on demand. The average is kept up to date with the
/// values in the collection.
/// Demonstrates **encapsulation** in Rust.
pub struct AveragedCollection {
    // Struct is pub so other can use it
    // Internal members are *private* and are accessed only via the public
    // methods defined below. This is to enforce the invariant that the average
    // is updated whenever the collection is updated.
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    /// Add a value into the collection. `self.average()` will reflect this
    /// change.
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    /// Remove a value from the collection. `self.average()` will reflect this
    /// change.
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(v) => {
                self.update_average();
                Some(v)
            }
            None => None,
        }
    }

    /// Return the arithmetic mean of the values in the collection. This is
    /// cached and not re-computed on demand.
    pub fn average(&self) -> f64 {
        self.average
    }

    /// Re-calculate the cached average value and update `self.value`. This is
    /// kept private to encapsulate the implementation of how the average is
    /// calculated and tracked - allowing it to be changed without callers of
    /// this struct needing to change.
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // This can contain values of multiple *different* types, so long as they
    // implement Draw
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    /// Draw all the current components on the screen.
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Button {
    fn to_str(&self) -> String {
        let n_outside_rows = 2;
        let n_outside_cols = 2;
        let n_non_blank_rows = n_outside_rows + 1;
        let inner_width = (self.width - n_outside_cols) as usize;
        let n_blank_rows = if self.height > n_outside_rows + 1 {
            self.height - n_non_blank_rows
        } else {
            3
        };
        let outside_row = format!("+{:-<1$}+", "", inner_width);
        let label_row = format!("|{:^1$}|", self.label, inner_width);
        let blank_row = format!("|{:<1$}|", "", inner_width);

        (0..n_blank_rows)
            .map(|i| {
                if i == 0 || i == n_blank_rows - 1 {
                    &outside_row
                } else if i == n_blank_rows / 2 {
                    &label_row
                } else {
                    &blank_row
                }
            })
            .map(|s| s.clone())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("{}", self.to_str());
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl SelectBox {
    fn to_str(&self) -> String {
        let n_outside_rows = 2;
        let n_outside_cols = 2;
        let inner_width = (self.width - n_outside_cols) as usize;
        let min_height = self.options.len() as u32 + n_outside_rows;
        let n_blank_rows = if self.height > min_height {
            self.height - n_outside_rows - self.options.len() as u32
        } else {
            min_height
        };

        let outside_row = format!("+{:-<1$}+", "", inner_width);
        let blank_row = format!("|{:<1$}|", "", inner_width);
        let iter_blanks = || (0..n_blank_rows / 2).map(|_| blank_row.clone());
        let mut rows: Vec<String> = vec![];
        rows.push(outside_row.clone());
        rows.extend(iter_blanks());
        rows.extend(
            self.options
                .iter()
                .map(|s| format!("|{:^1$}|", s, inner_width)),
        );
        rows.extend(iter_blanks());
        rows.push(outside_row);
        rows.join("\n")
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{}", self.to_str());
    }
}

fn main() {
    // Create a Screen that contains component of different *concrete* types
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 50,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            // This would not compile! String does not implement the Draw trait
            // and can therefore not be used in self.components.
            // Box::new(String::from("Hello")),
        ],
    };

    screen.run();
}
