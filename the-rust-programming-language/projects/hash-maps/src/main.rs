use std::collections::HashMap;

fn main() {
    // Construct 'manually'
    // String: i32
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Construct using collect on a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // Ownership
    // Types implementing Copy are copied into the hash map
    let one: i32 = 1;
    let also_one: i64 = 1;

    let mut map = HashMap::new();
    map.insert(one, also_one);
    // one and also_one are still valid here

    // Types without Copy are transferred to be owned by the hash map
    let s1 = String::from("foo");
    let s2 = String::from("bar");

    let mut map = HashMap::new();
    map.insert(s1, s2);
    // s1 and s2 are invalid from here on

    // Accessing values

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Individually using get
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Iterating over key, value pairs
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // Inserting

    // Overwrite existing
    scores.insert(String::from("Yello"), 100);

    // Only insert a value if key not present
    scores.or_insert(String::from("Blue"), 20); // Will not update scores
}
