use crate::log;
use crate::number::read_i32_array;

pub fn read_string(ptr: *mut u8, len: usize) -> String {
    let vec = unsafe { core::slice::from_raw_parts(ptr, len) };
    let str = String::from_utf8_lossy(vec).to_string();
    crate::free(ptr, len);
    str
}

pub fn write_string(str: &str) -> *const usize {

    let chars = str.chars().collect::<Vec<char>>();
    let str_ptr = chars.as_ptr() as usize;
    let str_len = chars.len();
    let header = vec![str_ptr, str_len * 4];

    log(&format!("str_ptr = {} len = {}", str_ptr, str_len * 4));
    let ptr = header.as_ptr();
    core::mem::forget(header);
    core::mem::forget(chars);

    ptr
}

pub fn write_string_arr(str_vec: Vec<String>) -> *const usize {
    let mut str_headers: Vec<*const usize> = Vec::new();
    for s in str_vec {
        str_headers.push(write_string(&s));
    }
    let header = vec![str_headers.as_ptr() as usize, str_headers.len()];

    let h_ptr = header.as_ptr();
    core::mem::forget(str_headers);
    core::mem::forget(header);

    h_ptr
}

pub fn read_string_arr(ptr: *const i32) -> Vec<String> {
    let str_headers = unsafe { core::slice::from_raw_parts(ptr, 2) };
    let mut str_vec: Vec<String> = Vec::new();

    for h in str_headers {
        let header = read_i32_array(*h as *const i32, 2);
        str_vec.push(read_string(header[0] as *mut u8, header[1] as usize));
    }

    crate::free(ptr as *mut u8, 2 * 4);

    str_vec
}