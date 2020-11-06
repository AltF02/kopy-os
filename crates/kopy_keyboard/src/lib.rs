#![no_std]

use kopy_core::{print, vga_buffer::WRITER};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

pub fn handle_key_event(mut port: Port<u8>) {
    let mut keyboard = KEYBOARD.lock();

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => match character {
                    '\u{8}' => interrupts::without_interrupts(|| {
                        WRITER.lock().delete_last_character();
                    }),
                    '\n' => interrupts::without_interrupts(|| {
                        ksh::handle_line(WRITER.lock().buffer);
                        ksh::new_line();
                    }),
                    _ => print!("{}", character),
                },
                DecodedKey::RawKey(key) => match key {
                    _ => print!("{:?}", key),
                },
            }
        }
    }
}
