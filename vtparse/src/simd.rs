#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_mm256_set1_epi8, _mm256_loadu_si256, __m256i, _mm256_cmpeq_epi8, _mm256_or_si256, _mm256_movemask_epi8};

#[cfg(target_arch = "x86_64")]
/// # Safety
///
/// This function is unsafe because it uses AVX2 intrinsics. The caller must ensure
/// that the `avx2` feature is supported by the CPU.
#[target_feature(enable = "avx2")]
#[must_use] 
pub unsafe fn find_escape_avx2(data: &[u8]) -> Option<usize> {
    let esc = _mm256_set1_epi8(0x1b_u8 as i8);
    let bel = _mm256_set1_epi8(0x07_u8 as i8);
    let st = _mm256_set1_epi8(0x9c_u8 as i8);

    let mut i = 0;
    while i + 32 <= data.len() {
        unsafe {
            let chunk = _mm256_loadu_si256(data[i..].as_ptr().cast::<__m256i>());
            let eq_esc = _mm256_cmpeq_epi8(chunk, esc);
            let eq_bel = _mm256_cmpeq_epi8(chunk, bel);
            let eq_st = _mm256_cmpeq_epi8(chunk, st);

            let match_mask = _mm256_or_si256(_mm256_or_si256(eq_esc, eq_bel), eq_st);
            let mask = _mm256_movemask_epi8(match_mask);

            if mask != 0 {
                return Some(i + mask.trailing_zeros() as usize);
            }
        }
        i += 32;
    }

    data[i..].iter().position(|&b| b == 0x1b || b == 0x07 || b == 0x9c).map(|pos| i + pos)
}

#[inline(always)]
#[must_use] 
pub fn find_first_escape(data: &[u8]) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { find_escape_avx2(data) };
        }
    }
    data.iter().position(|&b| b == 0x1b || b == 0x07 || b == 0x9c)
}
