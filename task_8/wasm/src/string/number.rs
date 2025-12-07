
pub fn write_i32_numbers(numbers: Vec<i32>) -> *const usize {
    let ptr = numbers.as_ptr() as *mut usize;
    let len = numbers.len();
    let header = vec![ptr as usize, len];
    let h_ptr = header.as_ptr();
    let h_len = header.len();

    core::mem::forget(header);
    core::mem::forget(numbers);

    h_ptr
}
