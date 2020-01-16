//! Demonstration of using the `Mutex<T>` smart pointer for *shared-state*
//! concurrency as well as `Arc<T>` to enable multiple threads to share a
//! `Mutex<T>` The Rust type system enforces the invariants required for a
//! mutex to safely guard shared memory:
//! 1. You must attempt to acquire the lock before using the data.
//! 2. When you’re done with the data that the mutex guards, you must unlock the
//! data so other threads can acquire the lock.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Basic API

    let m = Mutex::new(5);

    {
        let mut num = m
            // Acquire the lock in order to access the data it guards
            // This blocks until lock can be acquired
            .lock()
            // Panic if another thread holding the lock panics - there is no
            // way to acquire the lock in this case
            // Returns a MutexGuard smart pointer if unwrap succeeds
            .unwrap();
        // Mutate the shared value
        *num = 6;
    } // num MutexGuard pointer is dropped and the lock automatically released

    // Lock has been released, able to safely print out the value of m
    println!("m = {:?}", m);

    // Sharing Mutex<T> between threads using Arc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Should be 10
    println!("Result: {}", *counter.lock().unwrap());
}
