use std::mem::{size_of, transmute};
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::{ReadProcessMemory, VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};

unsafe fn dll_inject(prochandle: *mut c_void, size: usize) {
    let va_handle = VirtualAllocEx(
        prochandle,
        null_mut(),
        size,
        0x1000,
        0x40
    );

    WriteProcessMemory(
        prochandle,
        va_handle,
        size as *const c_void,
        size,
        null_mut()
    );

    // Gavno kakoeto sdelal
    // let kernel_handle = GetModuleHandleA("kernel32.dll\0".as_ptr() as *const i8);
    // let lib_handle = GetProcAddress(kernel_handle, "DLLInjectMain\0".as_ptr() as *const i8);

    // let thread_handle = CreateRemoteThread(
    //     prochandle,
    //     null_mut(),
    //     0,
    //     Some(transmute(test(prochandle) as *const i8)),
    //     prochandle,
    //     0,
    //     null_mut()
    // );

    // println!("{:?}", thread_handle);

    test(prochandle);
}

// This is a test inject function for get car Id in Need For Speed: Most Wanted;
unsafe fn test(prochandle: *mut c_void) {
    let mut bytes_read: usize = 0;
    let mut buffer: [u8; size_of::<u32>()] = [0; size_of::<u32>()];

    ReadProcessMemory(
        prochandle,
        0x009BA088 as *const c_void,
        buffer.as_mut_ptr() as *mut c_void,
        buffer.len(),
        &mut bytes_read as *mut usize
    );

    println!("DEBUG: {} | {:?}", bytes_read, buffer); // Where first index in buffer -> id car
}

fn main() {
    unsafe {
        let pid = 17604; // id process
        let proc_handle = OpenProcess(0x001FFFFF, 0, pid);
        dll_inject(proc_handle, 1024);
    };
}
