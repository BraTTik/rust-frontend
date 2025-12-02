mod allocations;

use allocations::*;

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
    let header = vec![str.as_ptr() as usize, str.len()];

    log(&format!("{:?}", header));
    let ptr = header.as_ptr();
    let len = header.len();
    core::mem::forget(header);
    store_allocation(ptr, len);

    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *const usize {
    let vec = Vec::with_capacity(size);
    let ptr = vec.as_ptr();
    core::mem::forget(vec);
    store_allocation(ptr, size);

    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut usize) {
    let cap = get_cap(ptr);
    unsafe { Vec::from_raw_parts(ptr, 0, cap); }
}

#[unsafe(no_mangle)]
pub extern "C" fn log_string(ptr: *mut u8, len: usize) {
    let str = read_string(ptr, len);
    log(&str);
}

pub fn read_string(ptr: *mut u8, len: usize) -> String {
    let vec = unsafe { core::slice::from_raw_parts(ptr, len) };
    String::from_utf8_lossy(vec).to_string()
}

#[unsafe(no_mangle)]
pub extern "C" fn join() {}