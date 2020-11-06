#![no_std]
#![no_main]

use kopy_core::vga_buffer::{Writer, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};
use kopy_core::{print, println};

pub fn init() {
    clear_screen();

    println!("Welcome to kopy, this is an work in progress OS");
    print!("\n");
    println!("For more info on how to run ksh enter help");
    print!("\n\n");

    print!("$ ")
}

fn clear_screen() {
    let mut writer = WRITER.lock();
    for row in 1..BUFFER_HEIGHT {
        writer.clear_row(row);
    }
}
