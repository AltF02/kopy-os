#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kopy_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kopy_core::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kopy_os::init();
    test_main();

    loop {}
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kopy_os::test_panic_handler(info)
}
