//! # In-memory key-value store command line application
//!
//! From the perspective of this binary `libactionkv` is treated the same as
//! any other 3rd party crate - even though it's within the same project.
//!
//! ```text
//! └── end user
//!     | [interacts with]
//!     └── akv_mem[.exe]
//!         | [compiles from]
//!         └── src/bin.rs
//!             | [imports]
//!             └──  libactionkv
//!                  | [compiles from]
//!                  └── src/lib.rs
//! ```
extern crate libactionkv;

use libactionkv::ActionKV;

// Platform specific help messages
#[cfg(target_os = "windows")]
const USAGE: &'static str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &'static str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    // Not all commands require a value e.g. `get`
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("Unable to open FILE");
    store.load().expect("Unable to load data");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            // TODO: Return old value if it exists?
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        }
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        }
        _ => eprintln!("{}", &USAGE),
    }
}
