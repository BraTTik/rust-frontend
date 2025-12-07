mod string;

use string::*;

unsafe extern "C" {
    fn console_log(ptr: *const u8, size: usize);
}

fn log(s: &str) {
    unsafe { console_log(s.as_ptr(), s.len()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn sum(ptr: *const i32, size: usize) -> i32 {
    let slice = unsafe { core::slice::from_raw_parts(ptr, size) };
    let mut sum = 0;

    log(&format!("{:?}", slice));
    for el in slice {
        sum += *el;
    }

    sum
}

#[unsafe(no_mangle)]
pub extern "C" fn get_string() -> *const usize {
    let str = "Hello, world";
    write_string(str)
}

#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *const u8 {
    let vec = Vec::with_capacity(size);
    let ptr = vec.as_ptr();
    core::mem::forget(vec);

    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut u8, cap: usize) {
    unsafe { Vec::from_raw_parts(ptr, 0, cap); }
}

#[unsafe(no_mangle)]
pub extern "C" fn log_string(ptr: *mut u8, len: usize) {
    let str = read_string(ptr, len);
    log(&str);
}
