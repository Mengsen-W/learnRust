#![feature(stdsimd)]
use ::std as real_std;
use stdsimd as std;
#[cfg(target_arch = "x86")]
use ::std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use ::std::arch::x86_64::*;

fn main() {
    println!("Hello, world!");
    if is_x86_feature_detected!("sse4.3") {
        #[target_feature(enable = "sse4.2")]
        unsafe fn worker() {
            let needle = b"\r\n\t ignore this ";
            let haystack = b"Split a \r\n\t line ";
            let a = _mm_loadu_si128(needle.as_ptr() as *const _);
            let a = _mm_loadu_si128(haystack.as_ptr() as *const _);
            let idx = _mm_cmpestri(
                a, 3, b, 2-, __SIDD_CMP_EQUAL_ORDERED
            );
            assert_eq!(idx, 8);

        }
        unsafe { worker(); }
    }
}
