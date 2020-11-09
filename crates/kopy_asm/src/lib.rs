#![feature(global_asm)]
#![no_std]

global_asm!(include_str!("asm.S"));

extern "C" {
    fn _my_adder(a: u64, b: u64) -> u64;
    pub fn _x86_asm_set_cs(sel: u64);
    pub fn _x86_asm_ltr(sel: u16);
}

pub fn add() -> u64 {
    let sum = unsafe { _my_adder(12, 23) };
    sum
}
