//! An OS kernel that prints text to the screen
//!
//! Note: This is an improved version of `fledgeos-2` that writes some text
//! to the screen instead of a simple coloured block.
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
/// [VGA-compatible text mode](https://en.wikipedia.org/wiki/VGA-compatible_text_mode)
/// 16 colour palette.
/// - 3 bits for main 8 colours
/// - Foreground colours include a "bright" variant
///
/// Values are forced to a single byte representation (e.g. the compiler will
/// not optimise the in memory representation) to enable them to be written
/// directly to the VGA framebuffer.
#[allow(unused)] // Not all colours will be used
#[derive(Clone, Copy)]
#[repr(u8)] // Ensure the compiler represents each value as a single byte
enum Colour {
    Black = 0x0,
    White = 0xF,
    Blue = 0x1,
    BrightBlue = 0x9,
    Green = 0x2,
    BrightGreen = 0xA,
    Cyan = 0x3,
    BrightCyan = 0xB,
    Red = 0x4,
    BrightRed = 0xC,
    Magenta = 0x5,
    BrightMagenta = 0xD,
    Brown = 0x6,
    Yellow = 0xE,
    Gray = 0x7,
    DarkGray = 0x8,
}

/// Interface to the VGA framebuffer.
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
/// ```
struct Cursor {
    position: isize,
    foreground: Colour,
    background: Colour,
}

impl Cursor {
    /// Return the foreground & background colours as a single byte
    /// - Foreground = lower 4 bits
    /// - Background = upper 4 bits
    fn colour(&self) -> u8 {
        // Foreground occupies the *lower* 4 bits
        let fg = self.foreground as u8;
        // Shift background colour to occupy the upper 4 bits
        let bg = (self.background as u8) << 4;
        // Merge foreground & background together
        fg | bg
    }

    /// Print `text` to the screen.
    /// TODO: Use a type that guarantees correct encoding rather than a raw
    /// byte stream. The authors chose this for expediency.
    fn print(&mut self, text: &[u8]) {
        let color = self.colour();

        // Bootloader sets up address 0xb8000 as the start of the VGA framebuffer
        let framebuffer = 0xb8000 as *mut u8;

        // Write characters to the framebuffer as `[character, color]` 'pairs'.
        for &character in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(character);
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2;
        }
    }
}

/// OS kernel entrypoint - prints text to the screen.
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
//
// Prevent rust from renaming this function in the compiled binary to allow
// the linker to interface with the binary at a known point
#[no_mangle]
// Use C compilation conventions to allow the function to be accessed by other
// programs in a standard interface
pub extern "C" fn _start() -> ! {
    let text = b"Welcome to FledgeOS!";

    let mut cursor = Cursor {
        position: 0,
        foreground: Colour::BrightCyan,
        background: Colour::Black,
    };
    cursor.print(text);

    // Do nothing, user can kill the process
    loop {
        // Notify CPU that there is no work to be done. CPU will resume
        // operating when an interrupt triggers a new action (not present here)
        hlt();
    }
}
