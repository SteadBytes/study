//! # Inspecting a program's memory via the Windows API
//! Scan the memory of a program whilst it's running to demonstrate virtual
//! memory mapping.
extern crate kernel32;
extern crate winapi;

use winapi::{
    DWORD, // u32
    HANDLE,
    LPSYSTEM_INFO,
    LPVOID, // Long void pointer
    MEMORY_BASIC_INFORMATION,
    PVOID,  // Void pointer
    SIZE_T, // usize
    SYSTEM_INFO,
};

fn main() {
    // Initialised within unsafe blocks, *define* outside to make accessible in
    // the outer scope
    let this_pid: DWORD;
    let this_proc: HANDLE;
    let min_app_addr: LPVOID;
    let max_app_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEMORY_BASIC_INFORMATION;

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

    // Ensure that all memory is initialised
    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    // Perform windows system calls
    unsafe {
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        kernel32::GetSystemInfo(&mut proc_info as LPSYSTEM_INFO);
    };

    min_app_addr = proc_info.lpMinimumApplicationAddress;
    max_app_addr = proc_info.lpMaximumApplicationAddress;

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_app_addr, max_app_addr);

    // Scan through the address space
    loop {
        let rc: SIZE_T = unsafe {
            // Retrieve information about a segment of the running programs
            // current memory address space
            kernel32::VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEMINFO_SIZE as SIZE_T)
        };

        if rc == 0 {
            break;
        }

        println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
    }
}
