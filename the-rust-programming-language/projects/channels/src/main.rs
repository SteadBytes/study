//! Demonstration of message passing concurrency using channels; generating
//! values on one thread and sending them to another thread via a channel to be
//! printed out.
use std::sync::mpsc; // mpsc = Multiple Producer Single Consumer
use std::thread;
use std::time::Duration;

fn main() {
    println!("Sending single value via a channel:");

    // Common abbreviations for (transmitter, receiver)
    let (tx, rx) = mpsc::channel();

    // Transmit into the channel from a new thread
    thread::spawn(move || {
        let val = String::from("hi");
        // Returns Result<T, E>, the error case occurs if the receiving end has
        // been dropped and there's nowhere to send the value to.
        // Using unwrap to panic in the error case, in real code the error
        // would be handled properly.
        tx.send(val).unwrap();
        // This won't compile! tx.send takes ownership of val - transferring
        // the ownership to rx. If the compiler allow this, a data race could
        // occur here due to using the value after it has been sent to another
        // thread.
        // println!("val is {}", val);
    });

    // Block until a value is received.
    // Returns Result<T, E>, the error case occurs if the transmitting end of
    // the channel closes.
    // Alternatively, rx.try_recv can be used if we do not want to block. This
    // would generally be used in some sort of "polling loop" when a thread has
    // other work to do in between receiving new values.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    println!("\nSending multiple values via a channel:");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            // Sleep to demonstrate the main thread waiting
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Use rx as an interator, printing each value as it is received. Iteration
    // stops when rx is closed. Demonstrates that the main thread is waiting
    // for new values to be received - the values should be printed out at ~1s
    // intervals due to the thread::sleep above.
    for received in rx {
        println!("Got: {}", received);
    }

    println!("\nMultiple producers:");

    // The transmitter end of the channel can be *cloned* to allow multiple
    // producers to send values into the same channel.

    let (tx, rx) = mpsc::channel();

    // Clone transmitting end of channel to send values from one thread
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            // Sleep to demonstrate the main thread waiting
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Use original reference to transmitting end to send values from another
    // thread
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            // Sleep to demonstrate the main thread waiting
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Received value order is **nondeterministic** - execution of each producer
    // thread is interleaved.
    for received in rx {
        println!("Got: {}", received);
    }
}
