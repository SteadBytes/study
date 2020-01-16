//! Demonstration of the basic std::thread API
use std::thread;
use std::time::Duration;

fn main() {
    interleaving();
    println!("");
    join();
    println!("");
    move_closures();
}

fn interleaving() {
    println!("Thread interleaving:");

    // This thread will likely be stopped prematurely as the main thread
    // exits before it completes. There is also no guarantee that it will ever
    // run as there is  not guarantee on which order threads will run.
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hello {} from the spawned thread! (interleaving)", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hello number {} from the main thread! (interleaving)", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn join() {
    println!("Using join to wait for threads to finish:");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello {} from the spawned thread! (join)", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Blocks the main thread until handle has run to completion.
    // Main thread and spawned thread outputs won't be interleaved.
    handle.join().unwrap();

    for i in 1..5 {
        println!("Hello number {} from the main thread! (join)", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn move_closures() {
    println!("Using move closures to use data from one thread in another:");

    let v = vec![1, 2, 3];

    // This won't compile! Rust infers that the closure is borrowing v, however
    // it may be dropped before the spawned thread is finished using that
    // borrowed reference. If the compiler allowed this it would introduce a
    // data race.
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // Instead, must use the move keyword before the closure to force ownership
    // of the values it's using to be transferred to the closure.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
