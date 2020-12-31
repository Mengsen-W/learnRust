fn add_quickly_fallback(a: &[u8], b: &[u8], c: &mut [u8]) {
    for ((a, b), c) in a.iter().zip(b).zip(c) {
        *c = *a + *b;
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn add_quickly_avx2(a: &[u8], b: &[u8], c: &mut [u8]) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            println!("support avx2");
            return unsafe { add_quickly_avx2(a, b, c) };
        }
    }
    add_quickly_fallback(a, b, c)
}

fn main() {
    let mut dst = [0, 2];
    add_quickly(&[1, 2], &[2, 3], &mut dst);
    assert_eq!(dst, [3, 5]);
}
