//! Allocating and de-allocating memory on the heap via `Box<T>`
use std::mem::drop;

fn main() {
    // Allocate values on the heap
    // Each `Box` will be stack allocated - containing a pointer to the actual
    // data (e.g. `1`) on the heap
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    // Dereference `Box` pointers to access data on the heap
    let result1 = *a + *b + *c;

    // Drop `a`, freeing up memory for use by other resources
    // Both the pointer on the stack and data in the heap may not actually be
    // deleted at this point, however they will be marked as available and
    // inaccessible from within safe Rust
    drop(a);
    // Allocate another value on the heap - in *theory* this could (e.g. it's
    // safe to do so as it has been freed) be stored at the same memory
    // location as `a` previously, though unlikely
    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("{} {}", result1, result2);
}
