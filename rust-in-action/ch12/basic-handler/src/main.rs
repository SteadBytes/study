//! Demonstration of signal handling with global variables.
//!
//! Signal handlers allow an application to perform some operations in response
//! to certain signals.
//! - e.g. close database connection on a `SIGTERM`.
//!
//! Signal handlers are usually implemented as simple functions which modify a
//! **global flag**. The main application loop checks said flag at regular
//! intervals and acts accordingly if set. This is because the behaviour of
//! handlers is **constrained by time and scope**:
//! - Scope:
//!   - Isn't able to access any non-global information from the application
//!     - `f(i32) -> ()` function signature
//!   - Must avoid executing code that may itself generate signals
//! - Time; must execute quickly:
//!   - Whilst execting, other signals of the same type are blocked from being
//!     handled
//!   - Reduces the likelihood that they are operating alongside *another*
//!     signal of the same type
//!
//! This program demonstrates signal handling by raising `SIGUSR1` and `SIGTERM`
//! signals and handling them accordingly:
//! - Print the string "SIGUSR1" on a `SIGUSR1`
//! - Print the string "SIGTERM" *and* terminate the application on a `SIGTERM`
//!

//! Signal handlers:
//! - `SIGUSRS`
#![cfg(not(windows))]

use libc::{SIGTERM, SIGUSR1};
use std::{thread::sleep, time};

/// Global flag indicating that the application should terminate.
/// - *Checked* by the main application loop
/// - *Set* by signal handlers
static mut SHUT_DOWN: bool = false;

///
///
fn main() {
    // Set up handlers as early as possible to ensure all signals are caught
    register_signal_handlers();

    let delay = time::Duration::from_secs(1);

    // Main loop:
    // - Checks the global `SHUT_DOWN` flag and, if set, terminates the loop.
    // - Raises `SIGUSR1` on the first 2 iterations
    // - Raises `SIGTERM` on all subsequent iterations
    for i in 1_usize.. {
        println!("{}", i);

        unsafe {
            if SHUT_DOWN {
                println!("*");
                return;
            }
        }

        sleep(delay);

        let signal = if i > 2 { SIGTERM } else { SIGUSR1 };
        unsafe {
            libc::raise(signal);
        }
    }
    unreachable!();
}

fn register_signal_handlers() {
    unsafe {
        libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
    }
}

/// Handle a `SIGTERM` signal by printing the string "SIGTERM" and setting the
/// `SHUT_DOWN` flag.
#[allow(dead_code)]
fn handle_sigterm(_signal: i32) {
    // Immediately re-register handlers to minimise the chance of missing
    // subsquent signals
    register_signal_handlers();

    println!("SIGTERM");

    unsafe {
        SHUT_DOWN = true;
    }
}

/// Handle a `SIGUSR1` signal by printing the string "SIGUSR1".
#[allow(dead_code)]
fn handle_sigusr1(_signal: i32) {
    register_signal_handlers();

    println!("SIGUSR1")
}
