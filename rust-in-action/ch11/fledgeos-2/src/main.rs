//! An OS kernel that paints a block of color to the screen
//!
//! Note: This is an improved version of `fledgeos-1` that implements a
//! minimalist (noop) exception handling *personality function*. This is
//! required due to not using the standard library which normally provides
//! an implementation.
//!
//! Language items: Pluggable operations used by `rustc` that are provided by
//! libraries and not hard-coded into the core language.
//! - Inidcated to the compiler using `#[lang = "..."]` marker attribute.
//! - Lazy loaded by the compiler
//!     - e.g. an implementation of the `box_free` language item will not be
//!       required if one does not use `Box`.
//!
//! Personality function: Language item that defines a function to called on
//! each stack frame during stack unwinding ([`std::rt::unwind`](https://doc.rust-lang.org/1.3.0/std/rt/unwind/)
//! contains some useful information). The personality function determines
//! whether the current stack frame is able to handle the exception that caused
//! the unwind e.g. whether it can *catch* the exception.
//!
//! Run the kernel using `bootimage`:
//! ```sh
//! bootimage run --target=fledge.json
//! ```

// Nightly only functions provide by LLVM
#![feature(core_intrinsics)]
// Enable defining language items using `#[lang = "..."]` attribute
#![feature(lang_items)]
// Exclude the standard library to simplify compilation to new target
#![no_std]
// Don't use `main` as the program entrypoint (see _start)
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;
use x86_64::instructions::hlt;

/// Noop error handling personality function. Rigorous exception handling in
/// FledgeOS is not required therefore this implementation is a noop simply to
/// appease the compiler.
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

/// Basic panic handler that aborts process execution.
///
/// `rustc` requires programs to have a mechanism to deal with panics -
///  normall provided by the Rust standard library.
#[panic_handler] // Associate this function with any panics that occur
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        intrinsics::abort();
    }
}

/// OS kernel entrypoint - paints a block of cyan colour on the screen.
///
/// This is used as the entrypoint instead of the usual `main` function:
/// - OS kernel main loop never returns
///   - Nowhere for it to return *to*
/// - No error code to return (convention for normal programs)
///   - Again, nowhere to return the error code to
///
/// A bootloader "talks" directly to the CPU in order to start an OS kernel.
/// The linker expects a single `_start` symbol to be defined. This is linked
/// to a function defined by the kernel source code.
///
/// `_start()` has 3 jobs:
/// - Reset the system
///   - Clear registers, reset memory etc
/// - Call `main()`
/// - Call `_exit()` to clean up after `main()`
///
/// Display is an 80x25 grid of cells each represented by 2 bytes of memory
/// in the VGA framebuffer. In Rust syntax, these bytes are:
/// ```
/// struct VGACell {
///     is_blinking: u1,
///     background_colour: u3,
///     is_bright: u1,
///     character_colour: u3,
///     /// Uses [Code page 437](https://en.wikipedia.org/wiki/Code_page_437) encoding
///     character: u8,
/// }
///
/// /// 16 colour palette.
/// /// - 3 bits for main 8 colours
/// /// - Foreground colours include a "bright" variant
/// #[repr(u8)]
/// enum Colour {
///     Black = 0,
///     White = 8,
///     Blue = 1,
///     BrightBlue = 9,
///     Green = 2,
///     BrightGreen = 10,
///     Cyan = 3,
///     BrightCyan = 11,
///     Red = 4,
///     BrightRed = 12,
///     Magenta = 5,
///     BrightMagenta = 13,
///     Brown = 6,
///     Yellow = 14,
///     Gray = 7,
///     DarkGray = 15,
/// }
/// ```
//
// Prevent rust from renaming this function in the compiled binary to allow
// the linker to interface with the binary at a known point
#[no_mangle]
// Use C compilation conventions to allow the function to be accessed by other
// programs in a standard interface
pub extern "C" fn _start() -> ! {
    // Bootloader sets up address 0xb8000 as the start of the VGA framebuffer
    let framebuffer = 0xb8000 as *mut u8;
    unsafe {
        // Set the background to cyan
        framebuffer.offset(1).write_volatile(0x30);
    }
    // Do nothing, user can kill the process
    loop {
        // Notify CPU that there is no work to be done. CPU will resume
        // operating when an interrupt triggers a new action (not present here)
        hlt();
    }
}
