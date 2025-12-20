mod string;
mod number;
mod boolean;

use string::*;
use number::*;
use crate::boolean::{read_bool_arr, write_bool, write_bool_arr};

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
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    let mut vec = Vec::with_capacity(size);
    let ptr = vec.as_mut_ptr();
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

#[unsafe(no_mangle)]
pub extern "C" fn get_i32_array() -> *const usize {
    let a = vec![1, 2, 3, 4];
    write_i32_array(a)
}

#[unsafe(no_mangle)]
pub extern "C" fn sum_i32_array(ptr: *const usize, len: usize) -> i32 {
    let a = read_i32_array(ptr as *const i32, len);
    a.iter().sum()
}

#[unsafe(no_mangle)]
pub extern "C" fn sum_i64_array(ptr: *const usize, len: usize) -> i64 {
    let a = read_i64_array(ptr as *const i64, len);
    a.iter().sum()
}

#[unsafe(no_mangle)]
pub extern "C" fn sum_f32_array(ptr: *const usize, len: usize) -> f32 {
    let a = read_f32_array(ptr as *const f32, len);
    a.iter().sum()
}

#[unsafe(no_mangle)]
pub extern "C" fn sum_f64_array(ptr: *const usize, len: usize) -> f64 {
    let a = read_f64_array(ptr as *const f64, len);
    a.iter().sum()
}

#[unsafe(no_mangle)]
pub extern "C" fn pow_i32_array(ptr: *const usize, len: usize) -> *const usize {
    let a = read_i32_array(ptr as *const i32, len);
    let pow_a = a.iter().map(|x| x.pow(2)).collect::<Vec<i32>>();
    write_i32_array(pow_a)
}

#[unsafe(no_mangle)]
pub extern "C" fn join_str(ptr: *const usize) -> *const usize {
    let vec = read_string_arr(ptr as *const i32);

   let res = vec.join("");

    log(&res);
    log(&format!("{:?}", vec));
    write_string(&res)
}

#[unsafe(no_mangle)]
pub extern "C" fn print_bool_arr(ptr: *const usize) {
    let arr = read_bool_arr(ptr as *const i32);
    log(&format!("Rust {:?}", arr));
}

#[unsafe(no_mangle)]
pub extern "C" fn compare_two_num(a: i32, b: i32) -> i32 {
    write_bool(a == b)
}


#[unsafe(no_mangle)]
pub extern "C" fn get_bool_arr() -> *const usize {
    let arr = vec![true, false, true];
    write_bool_arr(arr)
}