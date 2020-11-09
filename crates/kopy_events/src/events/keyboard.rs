use crate::Notifier;
use kopy_core::vga_buffer::Buffer;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref KEYBOARD_NOTIFIER: Mutex<Notifier<KeyboardEvent>> = Mutex::new(Notifier::new());
}

pub struct KeyboardEvent {
    pub key: char,
}
