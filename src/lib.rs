use ctor::{ctor, dtor};
use libc::{c_char, c_void, dlsym, RTLD_NEXT};
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Write;
use std::mem::transmute;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

struct Event {
    path: String,
    event_type: String,
    duration: u32,
}

static mut EVENTS: Vec<Event> = Vec::new();

#[ctor]
fn init() {
    let timestamp: u32 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("")
        .subsec_nanos();
    let rec = Event {
        path: String::from("__PROCESS__"),
        event_type: String::from("init"),
        duration: timestamp,
    };
    unsafe { EVENTS.push(rec) };

    println!("Hello, world!");
}

#[dtor]
fn fini() {
    let timestamp: u32 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("")
        .subsec_nanos();
    let rec = Event {
        path: String::from("__PROCESS__"),
        event_type: String::from("fini"),
        duration: timestamp,
    };
    unsafe { EVENTS.push(rec) };

    let mut file = File::create("foo.txt").expect("");
    let _ = file.write_all(b"Hello, world!");

    println!("Hello, world!");
}

fn benchmark<F: Fn() -> i32>(function: F) -> (i32, u32) {
    let start = Instant::now();
    let ret = function();
    let duration: u32 = start.elapsed().subsec_nanos();

    return (ret, duration);
}

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

    let closure = || -> i32 { cast_func(pathname, flags, mode) };
    let (ret, duration) = benchmark(closure);

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    return ret;
}
