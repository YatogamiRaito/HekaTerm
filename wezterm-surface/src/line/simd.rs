#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_mm256_loadu_si256, __m256i, _mm256_xor_si256, _mm256_testz_si256};

#[cfg(target_arch = "x86_64")]
/// # Safety
///
/// This function is unsafe because it uses AVX2 intrinsics. The caller must ensure
/// that the `avx2` feature is supported by the CPU.
#[target_feature(enable = "avx2")]
#[must_use] 
pub unsafe fn lines_equal_avx2(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut i = 0;
    while i + 32 <= a.len() {
        unsafe {
            let chunk_a = _mm256_loadu_si256(a.as_ptr().add(i).cast::<__m256i>());
            let chunk_b = _mm256_loadu_si256(b.as_ptr().add(i).cast::<__m256i>());
            let xored = _mm256_xor_si256(chunk_a, chunk_b);
            if _mm256_testz_si256(xored, xored) == 0 {
                return false;
            }
        }
        i += 32;
    }
    
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    
    true
}

#[inline(always)]
#[must_use] 
pub fn lines_equal(a: &[u8], b: &[u8]) -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { lines_equal_avx2(a, b) };
        }
    }
    a == b
}
