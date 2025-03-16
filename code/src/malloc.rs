use std::os::raw::c_int;

const M_MMAP_THRESHOLD: c_int = -3;

unsafe extern "C" {
	fn malloc_trim(pad: usize) -> c_int;
	fn mallopt(param: c_int, value: c_int) -> c_int;
}

/// Prevents glibc from hoarding memory via memory fragmentation.
pub fn limit_mmap_threshold(threshold: i32) {
	unsafe {
		mallopt(M_MMAP_THRESHOLD, threshold as c_int);
	}
}

/// Asks glibc to trim malloc arenas.
pub fn trim(pad: usize) {
	unsafe {
		malloc_trim(pad);
	}
}
