
pub fn read_string(ptr: *mut u8, len: usize) -> String {
    let vec = unsafe { core::slice::from_raw_parts(ptr, len) };
    let str = String::from_utf8_lossy(vec).to_string();
    crate::free(ptr, len);
    str
}

pub fn write_string(str: &str) -> *const usize {
    let header = vec![str.as_ptr() as usize, str.len()];

    let ptr = header.as_ptr();
    core::mem::forget(header);

    ptr
}

