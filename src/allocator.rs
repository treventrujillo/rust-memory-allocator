use std::ptr;

pub fn alloc(len: usize) -> *mut u8 {
    unsafe {
        libc::mmap(
            ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        ) as *mut u8
    }
}

pub fn dealloc(addr: *mut u8, len: usize) -> i32 {
    unsafe { libc::munmap(addr as *mut libc::c_void, len) }
}

mod tests {
    use super::*;

    #[test]
    fn test_alloc() {
        let pointer = alloc(4096);
        assert_ne!(libc::MAP_FAILED as *mut u8, pointer);
    }
}
