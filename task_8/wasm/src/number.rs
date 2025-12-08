
fn write_arr<T>(a: Vec<T>) -> *const usize {
    let ptr = a.as_ptr();
    let len = a.len();
    let header = vec![ptr as usize, len];
    let h_ptr = header.as_ptr();

    core::mem::forget(header);
    core::mem::forget(a);

    h_ptr
}

fn read_arr<T: Copy>(ptr: *const T, len: usize) -> Vec<T> {
    let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
    let mut vec: Vec<T> = Vec::new();

    for el in slice {
        vec.push(*el);
    }

    vec
}

pub fn read_i32_array(ptr: *const i32, len: usize) -> Vec<i32> {
    let vec: Vec<i32> = read_arr(ptr, len);
    crate::free(ptr as *mut u8, len * 4);
    vec
}

pub fn write_i32_array(a: Vec<i32>) -> *const usize {
    write_arr(a)
}

pub fn read_i64_array(ptr: *const i64, len: usize) -> Vec<i64> {
    let vec: Vec<i64> = read_arr(ptr, len);
    crate::free(ptr as *mut u8, len * 8);
    vec
}

pub fn write_i64_array(a: Vec<i64>) -> *const usize {
    write_arr(a)
}

pub fn write_f32_array(a: Vec<f32>) -> *const usize {
    write_arr(a)
}

pub fn read_f32_array(ptr: *const f32, len: usize) -> Vec<f32> {
    let vec: Vec<f32> = read_arr(ptr, len);
    crate::free(ptr as *mut u8, len * 4);
    vec
}

pub fn write_f64_array(a: Vec<f64>) -> *const usize {
    write_arr(a)
}

pub fn read_f64_array(ptr: *const f64, len: usize) -> Vec<f64> {
    let vec: Vec<f64> = read_arr(ptr, len);
    crate::free(ptr as *mut u8, len * 8);
    vec
}