//! # Inspecting a program's memory via the Linux `procfs` pseudo-filesystem
//! Scan the memory of a program whilst it's running to demonstrate virtual
//! memory mapping.
//!
//! Unlike the Windows version, there is no need for `unsafe` system calls. I'm
//! using the `procfs` crate to provide a high-level API for `procfs` but this
//! could also be achieved without external crates by simply reading the files
//! in `/proc` e.g.
//!
//! ```
//! use std::fs::File;
//!  use std::io::{self, prelude::*, BufReader};
//!  use std::process;
//!
//!  fn main() {
//!      let this_pid = process::id();
//!      let this_proc = File::open(format!("/proc/{}", this_pid));
//!      let cpu_info = File::open("/proc/cpuinfo");
//!      let mem_info = File::open("/proc/meminfo");
//!      let mem_maps = File::open(format!("/proc/{}/maps", this_pid));
//!  }
//! ```
extern crate procfs;

use procfs::{process::Process, CpuInfo, Meminfo};

fn main() {
    let this_proc = Process::myself().unwrap();
    let this_pid = this_proc.pid();
    let cpu_info = CpuInfo::new().unwrap();
    let mem_info = Meminfo::new().unwrap();
    let mem_maps = this_proc.maps().unwrap();

    println!("{}", this_pid);
    println!("{:#?}", cpu_info);
    println!("{:#?}", mem_info);
    for m in mem_maps {
        println!("{:#?}", m);
    }
}
