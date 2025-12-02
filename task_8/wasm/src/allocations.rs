static N: usize = 4096;

static mut KEYS: [usize; N] = [0; N];
static mut CAPS: [usize; N] = [0; N];

pub fn store_allocation(ptr: *const usize, size: usize) {
    unsafe {
        let mut i = (ptr as usize) % N;

        loop {
            if CAPS[i] == 0 {
                CAPS[i] = size;
                KEYS[i] = ptr as usize;
                return;
            }
            i = (i + 1) % N;
        }
    }
}

pub fn get_cap(ptr: *const usize) -> usize {
    unsafe {
        let mut i = (ptr as usize) % N;
        loop {
            if KEYS[i] == ptr as usize {
                return CAPS[i]
            }
            if KEYS[i] == 0 {
                return 0;
            }
            i = (i + 1) % N;
        }
    }
}