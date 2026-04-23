#![no_std]

use core::arch::global_asm;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".bss")]
static mut plugin_module_runtime: [u8; 0xD0] = [0u8; 0xD0];

global_asm!(
    r#"
    .section ".text.crt0","ax"
    .global plugin_start
    .extern plugin_module_runtime
    .hidden plugin_module_runtime

    plugin_start:
        .word 0
        .word plugin_mod0 - plugin_start
        /* bytes 0x08-0x7F are overwritten by elf2nro32 with NroStart padding + NroHeader.
           Reserve 0x78 bytes so all real code is placed at VMA >= 0x80. */
        .space 0x78

    .section ".rodata.mod0"
    .global plugin_mod0
    plugin_mod0:
        .ascii "MOD0"
        .word  __dynamic_start__     - plugin_mod0
        .word  __bss_start__         - plugin_mod0
        .word  __bss_end__           - plugin_mod0
        .word  0
        .word  0
        .word  plugin_module_runtime - plugin_mod0
    "#
);

#[unsafe(no_mangle)]
pub extern "C" fn init() {}

#[unsafe(no_mangle)]
pub extern "C" fn fini() {}

#[cfg(not(test))]
#[panic_handler]
fn forge_panic(_info: &PanicInfo) -> ! {
    loop {}
}
