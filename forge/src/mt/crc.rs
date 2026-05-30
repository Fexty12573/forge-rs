#[cfg(target_arch = "arm")]
use core::arch::naked_asm;

/// Offers CRC32 hashing functionality, equivalent to that used by the game.
///
/// MT Framework hashes strings (type names, property names, resource paths,
/// etc.) with a hardware CRC32 and uses the result as a stable identifier. This
/// type reproduces that hash bit-for-bit using the ARMv8 `crc32b` instruction,
/// so values computed here match those produced by the game itself.
///
/// `MtCRC` is a zero-sized, stateless helper; all of its methods are
/// associated functions. The `init` argument is the starting CRC value, which
/// the game seeds with `0xFFFFFFFF` (see [`MtDti::make_id`] for an example).
///
/// [`MtDti::make_id`]: crate::mt::dti::MtDti::make_id
pub struct MtCRC;

impl MtCRC {
    /// Computes the CRC32 of `bytes`, starting from the seed `init`.
    #[inline]
    pub fn from_bytes(bytes: &[u8], init: u32) -> u32 {
        unsafe { Self::get_raw(bytes.as_ptr(), bytes.len(), init) }
    }

    /// Computes the CRC32 of a string's UTF-8 bytes, starting from the seed
    /// `init`.
    ///
    /// Note that no NUL terminator is included in the hash; only the `str`'s
    /// own bytes are hashed.
    #[inline]
    pub fn from_str(str: &str, init: u32) -> u32 {
        unsafe { Self::get_raw(str.as_ptr(), str.len(), init) }
    }

    /// Computes the CRC32 over `len` bytes starting at `ptr`, seeded with
    /// `crc`.
    ///
    /// This is the raw hashing routine that backs [`from_bytes`] and
    /// [`from_str`]; prefer those safe wrappers unless you are working with a
    /// raw pointer directly. It is exported with the C ABI so it can also be
    /// called from or hooked by native game code.
    ///
    /// # Safety
    ///
    /// `ptr` must be valid for reads of `len` bytes. A `len` of `0` is
    /// permitted and returns `crc` unchanged regardless of `ptr`.
    ///
    /// [`from_bytes`]: Self::from_bytes
    /// [`from_str`]: Self::from_str
    #[cfg(target_arch = "arm")]
    #[unsafe(naked)]
    pub unsafe extern "C" fn get_raw(ptr: *const u8, len: usize, crc: u32) -> u32 {
        // arch and extension declarations required for the crc32b instruction
        naked_asm!(
            r#"
            .arch armv8-a
            .arch_extension crc
            cmp r1, #0
            beq 2f
            1:
            ldrb r3, [r0], #1
            uxtb r3, r3
            crc32b r2, r2, r3
            subs r1, r1, #1
            bne 1b
            2:
            mov r0, r2
            bx lr
        "#
        );
    }

    /// Stub for non-ARM hosts so the crate compiles off-target.
    /// This crate only ever runs on the game's ARM target, where the [`naked_asm`]
    /// implementation above is used instead.
    #[cfg(not(target_arch = "arm"))]
    pub unsafe extern "C" fn get_raw(_ptr: *const u8, _len: usize, crc: u32) -> u32 {
        crc
    }
}
