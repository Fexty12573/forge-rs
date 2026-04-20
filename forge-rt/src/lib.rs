#![no_std]

use core::arch::global_asm;

#[used]
#[no_mangle]
#[link_section = ".bss"]
pub static mut plugin_module_runtime: [u8; 0xD0] = [0u8; 0xD0];

global_asm!(
    r#"
    .section ".text.crt0","ax"
    .global plugin_start
    .extern plugin_module_runtime

    plugin_start:
        .word 0
        .word plugin_mod0 - plugin_start

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

#[no_mangle]
pub extern "C" fn init() {}

#[no_mangle]
pub extern "C" fn fini() {}
