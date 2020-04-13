# Kernel - building an Operating System

## Pre-requisites

- QEMU: visit https://www.qemu.org/ for installation instructions.
- [`bootimage`](https://github.com/rust-osdev/bootimage) crate and it's
  dependencies:

```sh
cargo install cargo-xbuild
cargo install cargo-binutils
cargo install bootimage
rustup default nightly
rustup component add rust-src
rustup component add llvm-tools-preview
```

Verify installation of pre-requisites:

```sh
# Check sysroot toolchain is nightly
$ rustc --print sysroot
/home/ben/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu

# Check llvm files are installed
$ find $(rustc --print sysroot) -type f -name 'llvm-*' -printf '%f\n'
llvm-ar
llvm-strip
llvm-readobj
llvm-objdump
llvm-profdata
llvm-objcopy
llvm-size
llvm-nm
llvm-asm.html

# Check xbuild installed
$ cargo xbuild --help
Wrapper for `cargo build` that cross-compiles core, compiler_builtins and alloc
# ...
```

`bootimage` handles compilation of an OS kernel written is Rust and creating a
bootable disk image. High level steps include:

- Compiling Rust itself to the target platform.
  - The OS does not exist yet, so there is no existing Rust build for it!
- Compiling the OS kernel for the new target, using the newly compiled Rust.
- Compiling a bootloader capable of loading the new kernel.
- Executing the bootloader in a virtual environment which in turn runs the
  kernel.
