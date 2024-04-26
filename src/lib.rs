use libc::{c_char, c_void, dlsym, RTLD_NEXT};
use std::ffi::{CStr, CString};
use std::mem::transmute;

#[no_mangle]
pub extern "C" fn open(pathname: *const c_char, flags: i32, mode: u32) -> i32 {
    unsafe {
        let fname = CString::new("open").expect("Failed to allocate CString!");
        let fhandle = dlsym(RTLD_NEXT, fname.as_ptr());
        let cast_func = transmute::<*mut c_void, fn(*const c_char, i32, u32) -> i32>(fhandle);
        cast_func(pathname, flags, mode)
    }
}
