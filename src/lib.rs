use libc::{c_char, c_void, dlsym, RTLD_NEXT};
use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::time::Instant;

#[no_mangle]
pub extern "C" fn open(pathname: *const c_char, flags: i32, mode: u32) -> i32 {
    let path = unsafe { CStr::from_ptr(pathname) };

    let cast_func = unsafe {
        let fname = CString::new("open").expect("Failed to allocate CString!");
        let fhandle = dlsym(RTLD_NEXT, fname.as_ptr());
        if fhandle.is_null() {
            println!("Failed to load function open!");
            return -1;
        }
        transmute::<*mut c_void, fn(*const c_char, i32, u32) -> i32>(fhandle)
    };

    let start = Instant::now();
    let ret = cast_func(pathname, flags, mode);
    let duration: u32 = start.elapsed().subsec_nanos();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    return ret;
}
