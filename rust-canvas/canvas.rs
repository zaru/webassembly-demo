fn main() {}

#[no_mangle]
pub extern "C" fn update(len: usize, ptr: *mut bool, col: usize) {
    let row = len / col;
    let buf: &mut [bool] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    for x in 0..len {
        if x % 2 == 0 {
            buf[x] = true
        } else {
            buf[x] = false
        }
    }
}
