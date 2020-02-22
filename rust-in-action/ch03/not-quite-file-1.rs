//! # Using plain functions to experiment with an API
//! Mock version of a files API to experiment with how the API (not the
//! implementation) could work.
#![allow(unused_variables)] // Relax compiler warnings whilst experimenting

type File = String; // In place of actually implementing a new type

fn open(f: &mut File) -> bool {
    true // Not a real implementation
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)] // Relax compiler warnings whilst experimenting
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec![]);
    close(&mut f1);
}
