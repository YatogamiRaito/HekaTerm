/// Find the first occurrence of an escape character (`\x1b`, `\x07`, `\x9c`)
/// in the provided byte slice. This function uses AVX2 instructions on x86_64
/// processors that support it (requires std for runtime feature detection),
/// and falls back to a standard iterator search otherwise.
pub fn find_first_escape(data: &[u8]) -> Option<usize> {
    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    {
        if std::is_x86_feature_detected!("avx2") {
            return unsafe { find_escape_avx2(data) };
        }
    }

    // Fallback for no_std, non-x86_64, or x86_64 without AVX2
    data.iter()
        .position(|&b| b == 0x1b || b == 0x07 || b == 0x9c)
}

#[cfg(all(target_arch = "x86_64", feature = "std"))]
#[target_feature(enable = "avx2")]
unsafe fn find_escape_avx2(data: &[u8]) -> Option<usize> {
    use core::arch::x86_64::*;

    let len = data.len();
    if len == 0 {
        return None;
    }

    let ptr = data.as_ptr();
    let mut offset = 0;

    let esc = _mm256_set1_epi8(0x1b_u8 as i8);
    let bel = _mm256_set1_epi8(0x07_u8 as i8);
    let st = _mm256_set1_epi8(0x9c_u8 as i8);

    // Process blocks of 32 bytes
    while offset + 32 <= len {
        let chunk = unsafe { _mm256_loadu_si256(ptr.add(offset) as *const __m256i) };

        let cmp_esc = _mm256_cmpeq_epi8(chunk, esc);
        let cmp_bel = _mm256_cmpeq_epi8(chunk, bel);
        let cmp_st = _mm256_cmpeq_epi8(chunk, st);

        let match_mask = _mm256_or_si256(_mm256_or_si256(cmp_esc, cmp_bel), cmp_st);
        let bitmask = _mm256_movemask_epi8(match_mask) as u32;

        if bitmask != 0 {
            return Some(offset + bitmask.trailing_zeros() as usize);
        }

        offset += 32;
    }

    // Handle remaining bytes
    if offset < len {
        let rem = &data[offset..];
        if let Some(pos) = rem
            .iter()
            .position(|&b| b == 0x1b || b == 0x07 || b == 0x9c)
        {
            return Some(offset + pos);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_escape() {
        let data1 = b"hello\x1bworld";
        assert_eq!(find_first_escape(data1), Some(5));

        let data2 = b"no escapes here";
        assert_eq!(find_first_escape(data2), None);

        let data3 = b"\x07beep";
        assert_eq!(find_first_escape(data3), Some(0));

        let data4 = b"lots of text before \x9ccommand sequence";
        assert_eq!(find_first_escape(data4), Some(20));

        // Test exactly 32 bytes without escape
        let data5 = [b'A'; 32];
        assert_eq!(find_first_escape(&data5), None);

        // Test exactly 32 bytes with escape at the end
        let mut data6 = [b'A'; 32];
        data6[31] = 0x1b;
        assert_eq!(find_first_escape(&data6), Some(31));

        // Test more than 32 bytes
        let mut data7 = [b'B'; 40];
        data7[35] = 0x07;
        assert_eq!(find_first_escape(&data7), Some(35));
    }
}
