fn main() {}

fn diffusion(org_buf: &[bool], row: usize, col: usize) -> Vec<bool> {
    let mut buf = vec![false; org_buf.len()];
    let mut skip = vec![false; org_buf.len()];
    let len = row * col - 1;

    for x in 0..len {
        if org_buf[x] == true {
            buf[x] = true;
        }
    }

    for x in 0..len {
        if org_buf[x] == true && !skip[x]{
            if x > col {
                let top = x - col;
                if buf[top] == false {
                    skip[top] = true;
                    buf[top] = true;
                }
            }
            let right = x + 1;
            if buf[right] == false {
                skip[right] = true;
                buf[right] = true;
            }
            let bottom = x + col;
            if bottom < len {
                if buf[bottom] == false {
                    skip[bottom] = true;
                    buf[bottom] = true;
                }
            }
            if x > 0 {
                let left = x - 1;
                if buf[left] == false {
                    skip[left] = true;
                    buf[left] = true;
                }
            }
        }
    }

    buf
}

#[no_mangle]
pub extern "C" fn update(len: usize, ptr: *mut bool, col: usize) {
    let row = len / col;
    let buf: &mut [bool] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let pos: Vec<bool> = diffusion(buf, row, col);
    buf.clone_from_slice(pos.as_slice());
}
