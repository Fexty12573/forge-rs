use core::arch::naked_asm;

pub struct MtCRC;

impl MtCRC {
    #[inline]
    pub fn from_bytes(bytes: &[u8], init: u32) -> u32 {
        unsafe { Self::get_raw(bytes.as_ptr(), bytes.len(), init) }
    }

    #[inline]
    pub fn from_str(str: &str, init: u32) -> u32 {
        unsafe { Self::get_raw(str.as_ptr(), str.len(), init) }
    }

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
}
