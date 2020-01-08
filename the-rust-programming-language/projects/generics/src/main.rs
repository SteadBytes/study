// Blocks are used to separate each example
// Keeping it all in main() for simplicity when reading

fn main() {
    // Generic function arguments
    {
        // PartialOrd + Copy traits discussed in detail in later chapter
        // For now, accept that this function is generic over lists of any type
        // that meets those two constraints
        fn largest<T: PartialOrd + Copy>(l: &[T]) -> T {
            let mut largest = l[0];
            for &item in l.iter() {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
        let number_list = vec![25, 100, 42, 29];
        println!("The largest number is: {}", largest(&number_list));

        let char_list = vec!['z', 'a', 'b', 'c'];
        println!("The largest char is: '{}'", largest(&char_list));

        let str_list = vec!["hello", "world", "foo", "bar"];
        println!("The largest str is: \"{}\"", largest(&str_list));
    }

    // Generic struct members
    {
        // Homogeneous types
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 5.0, y: 10.0 };

        // Won't compile
        // let float_integer = Point { x: 5.0, y: 10 };
    }

    {
        // Heterogeneous types
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let integer_float = Point { x: 5, y: 10.0 };
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 5.0, y: 10.0 };
    }

    // Enums
    {
        enum ExcitingOption<T> {
            Stuff(T),
            Nowt,
        }

        enum ExcitingResult<T, E> {
            Woo(T),
            Crap(E),
        }
    }

    // Method definitions
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        // Only implemented for instances using f32
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
        // Won't compile as distance_from_origin not implemented for integer
        // println!("Distance from origin = {}", p.distance_from_origin());

        let p2 = Point { x: 3.0, y: 10.0 };
        println!("Distance from origin = {}", p2.distance_from_origin());
    }
}
