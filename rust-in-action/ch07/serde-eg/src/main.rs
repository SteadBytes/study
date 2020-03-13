//! # Converting a Rust struct to three different file formats with `serde`
//!
//! `serde` provides a framework for serialisation and desirialisation of Rust
//! values to and from different data formats. This program demonstrates basic
//! usage of `serde` by converting a simple struct to JSON, CBOR and bincode
//! formats.
#[macro_use]
extern crate serde_derive;

extern crate bincode;
extern crate serde;
extern crate serde_cbor;
extern crate serde_json;

// Use `serde_derive` macros to generate the code for converting `City` values
// between in-memory and on-disk representations.
#[derive(Serialize, Deserialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let london = City {
        name: String::from("London"),
        // https://en.wikipedia.org/wiki/London as of 09/03/2020
        population: 8900000,
        latitude: 51.509865,
        longitude: 0.1268,
    };

    // APIs are different due to each format being provided by separate crates
    // not directly part of the `serde` core.
    let as_json = serde_json::to_string(&london).unwrap();
    let as_cbor = serde_cbor::to_vec(&london).unwrap();
    let as_bincode = bincode::serialize(&london).unwrap();

    println!("json: {}", &as_json);
    println!("cbor: {:?}", &as_cbor);
    println!("cbor (as UTF-8): {:?}", String::from_utf8_lossy(&as_cbor));
    println!("bincode: {:?}", &as_bincode);
    println!(
        "bincode (as UTF-8): {:?}",
        String::from_utf8_lossy(&as_bincode)
    );
}
