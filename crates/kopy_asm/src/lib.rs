#![feature(global_asm)]
#![no_main]
#![no_std]

global_asm!(include_str!("add.S"));

extern "C" {
    fn _my_adder(a: u64, b: u64) -> u64;
}

pub fn add() -> u64 {
    let sum = unsafe { _my_adder(5, 10) };
    sum
}
