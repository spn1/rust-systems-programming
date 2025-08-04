use kernel32;
use winapi;

use winapi::{
    DWORD, // u32 in rust
    HANDLE, // A pointer to some opaque resource in windows. std::os::raw::c_void void point in rust
    LPVOID, // A pointer to some opaque resource in windows. std::os::raw::c_void void point in rust
    SIZE_T, // usize on machine (u64)
    LPSYSTEM_INFO, // A pointer to a SYSTEM_INFO struct
    SYSTEM_INFO, // Internal windows structs
    MEMORY_BASIC_INFORMATION as MEMINFO, // Internal windows structs
};

pub fn run() {
        let this_pid: DWORD;
        let this_proc: HANDLE;
        let min_addr: LPVOID;
        let max_addr: LPVOID;
        let mut base_addr: LPVOID;
        let mut proc_info: SYSTEM_INFO;
        let mut mem_info: MEMINFO; 

        const MEMINFO_SIZE: usize = std::mem::size_of::<MEMINFO>();

        unsafe { // Initialize memory with zeroes
            base_addr = std::mem::zeroed();
            proc_info = std::mem::zeroed();
            mem_info = std::mem::zeroed();
        }

        unsafe { // Make system calls
            this_pid = kernel32::GetCurrentProcessId();
            this_proc = kernel32::GetCurrentProcess();
            // Reads system into into provided struct (c-style) instead of returning it
            kernel32::GetSystemInfo(&mut proc_info as LPSYSTEM_INFO);
        }

        min_addr = proc_info.lpMinimumApplicationAddress;
        max_addr = proc_info.lpMaximumApplicationAddress;

        println!("{:?} @ {:p}", this_pid, this_proc);
        println!("{:?}", proc_info);
        println!("min: {:p}, max: {:p}", min_addr, max_addr);

        loop { // Scan through address space
            let rc: SIZE_T = unsafe {
                // Provides information about a specific segment of the running
                // program's memory address space
                kernel32::VirtualQueryEx(
                    this_proc,
                    base_addr,
                    &mut mem_info,
                    MEMINFO_SIZE as SIZE_T
                )
            };
                
            if rc == 0 {
                break
            }

            println!("{:#?}", mem_info);
            base_addr = ((base_addr as u64) + mem_info.RegionSize) as LPVOID;
        };
}