//! Printing out the address of several variables within a program to examine
//! its address space.
//!
//! This avoids the runtime panics of `memscan-1` and `memscan-2` by:
//! - Not dereferencing null pointers
//! - Only examining the addresses of values *known to exist* e.g. created by
//! this program
static GLOBAL: i32 = 1000;

/// Returns the memory address of a local variable created *within* this
/// function.
fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

fn main() {
    // Create values of various types including both stack and heap allocation
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();

    // Display addresses of above values
    // These will (very likely) be unique between program execution
    // Scattered across a *wide* address range. The total RAM usage by this
    // program is only a few Kb but they allocated across a large range of
    // *virtual addresses*
    println!("GLOBAL:    {:p}", &GLOBAL as *const i32);
    println!("local_str: {:p}", local_str as *const str);
    println!("local_int: {:p}", &local_int as *const i32);
    println!("boxed_int: {:p}", Box::into_raw(boxed_int));
    println!("boxed_str: {:p}", Box::into_raw(boxed_str));
    println!("fn_int:    {:p}", fn_int);
}
