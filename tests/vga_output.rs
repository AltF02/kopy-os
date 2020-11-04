#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kopy_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kopy_os::println;
use kopy_os::vga_buffer::{BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kopy_os::test_panic_handler(info)
}

#[test_case]
fn test_println_many() {
    for i in 0..200 {
        println!("test_println_many output {}", i)
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
