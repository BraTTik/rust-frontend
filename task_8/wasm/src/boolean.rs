use crate::log;
use crate::number::{read_i32_array, write_i32_array};

pub fn read_bool(ptr: i32) -> bool {
    match ptr {
        0 => false,
        _ => true
    }
}

pub fn write_bool(b: bool) -> i32 {
    match b {
        true => 1,
        false => 0
    }
}

pub fn read_bool_arr(ptr: *const i32) -> Vec<bool> {
    let header = read_i32_array(ptr, 2);
    let arr = read_i32_array(header[0] as *const i32, header[1] as usize);
    arr.iter().map(|x| read_bool(*x)).collect()
}

pub fn write_bool_arr(arr: Vec<bool>) -> *const usize {
    let v: Vec<i32> = arr.iter().map(|x| write_bool(*x)).collect();
    log(&format!("{:?}", v));
    let vec_len = v.iter().len();
    let ptr = write_i32_array(v);

    log(&format!("{:?}", vec![ptr as i32, vec_len as i32 * 4]));
    write_i32_array(vec![ptr as i32, vec_len as i32 * 4])
}