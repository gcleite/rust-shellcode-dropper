use std::ffi::CString;
use std::ptr::null_mut;

#[link(name = "user32")]
extern "stdcall" {
    fn MessageBoxA(hWnd: *mut libc::c_void, lpText: *const libc::c_char, lpCaption: *const libc::c_char, uType: libc::c_uint) -> libc::c_int;
}

fn main() {
    let text = CString::new("Hello, world!").unwrap();
    let caption = CString::new("Message Box").unwrap();

    unsafe {
        MessageBoxA(null_mut(), text.as_ptr(), caption.as_ptr(), 0);
    }
}
