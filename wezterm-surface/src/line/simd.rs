#[cfg(all(target_arch = "x86_64", feature = "std"))]
use core::arch::x86_64::{__m256i, _mm256_loadu_si256, _mm256_testz_si256, _mm256_xor_si256};

#[cfg(all(target_arch = "x86_64", feature = "std"))]
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
    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    {
        if std::is_x86_feature_detected!("avx2") {
            return unsafe { lines_equal_avx2(a, b) };
        }
    }
    a == b
}

/// Compute a fast hash of a byte slice representing a terminal line.
///
/// On x86_64 CPUs with SSE4.2, this uses the hardware `crc32c` instruction
/// for throughput of ~1 cycle per 4 bytes. On other architectures it falls
/// back to the FNV-1a scalar hash.
#[inline(always)]
#[must_use]
pub fn hash_line(data: &[u8]) -> u64 {
    #[cfg(all(target_arch = "x86_64", feature = "std"))]
    {
        if std::is_x86_feature_detected!("sse4.2") {
            return hash_line_crc32(data);
        }
    }
    hash_line_scalar(data)
}

/// Scalar FNV-1a hash fallback.
#[inline(always)]
fn hash_line_scalar(data: &[u8]) -> u64 {
    const FNV_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hash = FNV_BASIS;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Hardware CRC32 hash using the x86_64 `crc32` instruction from SSE4.2.
#[cfg(target_arch = "x86_64")]
fn hash_line_crc32(data: &[u8]) -> u64 {
    let mut crc: u64 = 0xFFFF_FFFF_FFFF_FFFF;
    let mut chunks = data.chunks_exact(8);
    for chunk in &mut chunks {
        let val = u64::from_le_bytes(chunk.try_into().unwrap());
        // SAFETY: guarded by is_x86_feature_detected!("sse4.2") in the caller
        unsafe {
            core::arch::asm!(
                "crc32 {crc}, {val}",
                crc = inout(reg) crc,
                val = in(reg) val,
            );
        }
    }
    // handle remaining bytes
    for &b in chunks.remainder() {
        let val = b as u64;
        unsafe {
            core::arch::asm!(
                "crc32 {crc:r}, {val:r}",
                crc = inout(reg) crc,
                val = in(reg) val,
            );
        }
    }
    crc ^ 0xFFFF_FFFF_FFFF_FFFF
}
