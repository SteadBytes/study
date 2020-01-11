fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![2, 4, 6];

    // Calling next on an iterator changes internal state that the iterator
    // uses to keep track of it's position in the sequence
    // -> must be mutable
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), Some(&6));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // Consuming adaptor -> consumes the iterator
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // Map is an iterator adaptor -> creates another (lazy) iterator
    // Collect consumes the lazy iterator returned from map into a vector
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter() // takes ownership of the vector to collect into a new vector
        .filter(|s| s.size == shoe_size) // closure capturing environment to filter by shoe size
        .collect() // collect into vector (consume iterator)
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 8,
            style: String::from("Trainer"),
        },
        Shoe {
            size: 6,
            style: String::from("Sandals"),
        },
        Shoe {
            size: 8,
            style: String::from("Dr Martens"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 8);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 8,
                style: String::from("Trainer")
            },
            Shoe {
                size: 8,
                style: String::from("Dr Martens")
            },
        ]
    );
}
// Create an iterator that counts up to 6
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn counter_calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn counter_using_other_iterator_trait_methods() {
    // Iterator trait has default implementations of other methods
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
