//! `sjlj` (`shortjump` & `longjump`): Using LLVM intrinsics to access OS
//! `longjmp` facilities to allow programs to escape their call stack.
//!
//! Note: See the `callstack` program from this chapter for comparison of how
//! the callstack operates normally.
//!
//! Intrinsics = functions made available via the *compiler* rather than as
//! part of the programming language itself. They provide functionality that
//! requires greater access to the environment within which a program is run.
//! - Specialist CPU instructions
//! - CPU specific error handling
//!     - `setjmp` & `longjmp`
#![feature(link_llvm_intrinsics)]
#![allow(non_camel_case_types)]
#![cfg(not(windows))]

use libc::{SIGALRM, SIGHUP, SIGQUIT, SIGTERM, SIGUSR1};
use std::mem;

const JMP_BUF_WIDTH: usize = mem::size_of::<usize>() * 8;
type jmp_buf = [i8; JMP_BUF_WIDTH];

/// Flag to indicate the program should shut down.
static mut SHUT_DOWN: bool = false;
/// Buffer to hold a checkpoint of the programs current register state when
/// `setjmp` is called.
static mut RETURN_HERE: jmp_buf = [0; JMP_BUF_WIDTH];
/// Max recursion depth before raising a signal
const MOCK_SIGNAL_DEPTH: usize = 3;

// Declare LLVM `setjmp` & `longjmp` intrinsic functions
extern "C" {
    #[link_name = "llvm.setjmp"]
    pub fn setjmp(a: *mut i8) -> i32;

    #[link_name = "llvm.longjmp"]
    pub fn longjmp(a: *mut i8, b: i32) -> ();
}

/// Return a pointer to `RETURN_HERE` `jmp_buf` casted correctly for use with
/// LLVM `setjmp` & `longjmp` intrinsics.
#[inline]
fn ptr_to_jmp_buf() -> *mut i8 {
    unsafe { &RETURN_HERE as *const i8 as *mut i8 }
}

/// Return to program state checkpointed in `RETURN_HERE` using `longjmp`.
#[inline]
fn return_early() {
    let buf_ptr = ptr_to_jmp_buf();
    unsafe { longjmp(buf_ptr, 1) };
}

fn register_signal_handler() {
    unsafe {
        libc::signal(SIGUSR1, handle_signals as usize);
    }
}

#[allow(dead_code)]
fn handle_signals(sig: i32) {
    // Immediately re-register handlers to minimise the chance of missing
    // subsquent signals
    register_signal_handler();

    let should_shut_down = match sig {
        SIGHUP => false,
        SIGALRM => false,
        SIGTERM => true,
        SIGQUIT => true,
        SIGUSR1 => true,
        _ => false,
    };

    unsafe {
        SHUT_DOWN = should_shut_down;
    }

    return_early();
}

fn print_depth(depth: usize) {
    println!("{}", "#".repeat(depth));
}

/// Recursive function that returns normally once `max_depth` is reached or
/// using a `longjmp` early return once `MOCK_SIGNAL_DEPTH` is reached.
fn dive(depth: usize, max_depth: usize) {
    unsafe {
        if SHUT_DOWN == true {
            println!("!");
            return;
        }
    }
    print_depth(depth);

    if depth >= max_depth {
        return;
    }

    // Raise a signal to trigger `longjmp` early return performed by
    // `handle_signals`.
    if depth == MOCK_SIGNAL_DEPTH {
        unsafe {
            libc::raise(SIGUSR1);
        }
    } else {
        dive(depth + 1, max_depth);
    }
    print_depth(depth);
}

fn main() {
    register_signal_handler();

    // Checkpoint the current program state for the early return to resume at.
    let return_point = ptr_to_jmp_buf();
    let rc = unsafe { setjmp(return_point) };
    // `setjmp` returns 0 if returning from a direct invocation and 1 if from
    // a call to `longjmp`. Thus if `dive` performs a `longjmp`, a second call
    // to `dive` is *avoided* and "early return!" is printed.
    if rc == 0 {
        dive(0, 10);
    } else {
        println!("early return!");
    }

    println!("finishing!")
}
