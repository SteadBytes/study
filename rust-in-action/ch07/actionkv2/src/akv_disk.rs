//! # Key-value store command line application with persistent indexing
//!
//! Improved implementation of `akv_mem` that stores it's index within the
//! on disk data file instead of re-building the index in-memory on each run.
//!
//! From the perspective of this binary `libactionkv` is treated the same as
//! any other 3rd party crate - even though it's within the same project.
//!
//! ```text
//! └── end user
//!     | [interacts with]
//!     └── akv_disk[.exe]
//!         | [compiles from]
//!         └── src/bin.rs
//!             | [imports]
//!             └──  libactionkv
//!                  | [compiles from]
//!                  └── src/lib.rs
//! ```
extern crate bincode;
extern crate libactionkv;

use libactionkv::{ActionKV, ByteStr, ByteString};
use std::collections::HashMap;

// Platform specific help messages
#[cfg(target_os = "windows")]
const USAGE: &'static str = "
Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &'static str = "
Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
";

/// Inserts `store.index` into the database on disk at key `index_key`.
///
/// This will clear the current in-memory index and is only intended to be
/// called at the start of the program after loading the store initially.
///
/// # Implementation Details
///
/// The index `HashMap` is serialised using the `bincode` format.
fn store_index_on_disk(store: &mut ActionKV, index_key: &ByteStr) {
    store.index.remove(index_key);
    // Load existing index
    let index_as_bytes = bincode::serialize(&store.index).unwrap();
    store.index = HashMap::new();
    store.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    // Not all commands require a value e.g. `get`
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("Unable to open FILE");
    store.load().expect("Unable to load data");
    store_index_on_disk(&mut store, INDEX_KEY);

    match action {
        "get" => {
            // Retrieve the saved index from disk
            let index_as_bytes = store.get(&INDEX_KEY).unwrap().unwrap();
            // Convert on-disk index into in-memory representation
            let index: HashMap<ByteString, u64> = bincode::deserialize(&index_as_bytes).unwrap();

            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(idx) => {
                    // Grab the data directly from the indexed file position
                    let kv = store.get_at(*idx).unwrap();
                    println!("{:?}", kv.value);
                }
            }
        }
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
