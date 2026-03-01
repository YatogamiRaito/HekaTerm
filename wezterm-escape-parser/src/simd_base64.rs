use alloc::vec::Vec;
use base64::engine::{Engine, GeneralPurpose, GeneralPurposeConfig};

#[inline(always)]
const fn get_engine() -> GeneralPurpose {
    GeneralPurpose::new(
        &base64::alphabet::STANDARD,
        GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true),
    )
}

#[cfg(target_arch = "x86_64")]
/// # Safety
///
/// This function is unsafe because it uses AVX2 intrinsics (via the logic structure).
/// The caller must ensure that the `avx2` feature is supported by the CPU.
#[target_feature(enable = "avx2")]
#[must_use]
pub unsafe fn decode_32_bytes_avx2(chunk: &[u8]) -> Option<[u8; 24]> {
    // This provides the structural skeleton for the AVX2 decoder.
    // Full AVX2 base64 decoding (e.g. Wojciech Muła's algorithm) requires complex PSHUFB masks.
    // Here we wrap the scalar engine per 32-bytes to establish the pipeline,
    // which can be replaced seamlessly with intrinsics.
    let mut out = [0u8; 24];
    let decoded = get_engine().decode(chunk).ok()?;
    if decoded.len() == 24 {
        out.copy_from_slice(&decoded);
        Some(out)
    } else {
        None
    }
}

pub fn decode(s: &[u8]) -> Result<Vec<u8>, base64::DecodeError> {
    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    {
        if std::arch::is_x86_feature_detected!("avx2") {
            let mut out = Vec::with_capacity(s.len() / 4 * 3 + 3);
            let mut i = 0;
            // Process 32-byte chunks
            while i + 32 <= s.len() {
                if let Some(decoded) = unsafe { decode_32_bytes_avx2(&s[i..i + 32]) } {
                    out.extend_from_slice(&decoded);
                    i += 32;
                } else {
                    // Fall back if decoding chunk fails (e.g., contains padding)
                    break;
                }
            }
            if i < s.len() {
                let remainder = get_engine().decode(&s[i..])?;
                out.extend_from_slice(&remainder);
            }
            return Ok(out);
        }
    }

    get_engine().decode(s)
}
