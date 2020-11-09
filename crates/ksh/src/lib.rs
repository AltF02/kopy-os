#![no_std]

extern crate alloc;

pub mod builtin;
pub mod repl;
pub mod shell;

use crate::builtin::{builtin_echo, builtin_eval, KshBuiltin};
use crate::repl::Repl;
use crate::shell::{KshCommand, KshOutput};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::str::FromStr;
use kopy_core::vga_buffer::{Buffer, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};
use kopy_core::{print, println};
use kopy_events::events::keyboard::{KeyboardEvent, KEYBOARD_NOTIFIER};
use x86_64::instructions::interrupts::without_interrupts;

pub fn init() {
    clear_screen();

    println!("Welcome to kopy, this is an work in progress OS");
    print!("\n");
    println!("For more info on how to run ksh enter help");
    print!("\n");
    println!("ASM Debug: {}\n", kopy_asm::add());

    KEYBOARD_NOTIFIER.lock().register(|e| match e.key {
        '\n' => shell::KshCommand::loop_interactive(),
        _ => print!("{}", e.key),
    });
    new_line();
}

fn clear_screen() {
    let mut writer = WRITER.lock();
    for row in 1..BUFFER_HEIGHT {
        writer.clear_row(row);
    }
}

fn new_line() {
    print!("$ ");
}

fn read_line() -> String {
    let mut command = { WRITER.lock().read_line() };
    command = command.replace("$", "").trim().to_string();
    // print!("\n");
    // println!("DEBUG: {:?}", command);
    // let command_split: Vec<&str> = command.split(' ').collect();
    // println!("DEBUG: {:?}\n", command_split);
    command
}

pub fn handle_line(_keyboard_event: &KeyboardEvent) {
    let command = read_line();
    let mut args = {
        let mut command_split: Vec<&str> = command.split(' ').collect();
        command_split.clone()
    };
    match args[0] {
        "echo" => {
            args.remove(0);
            println!("\n{}", args.join(" "));
        }
        _ => {}
    }
    new_line();
}

pub fn tokenize_command(c: String) -> KshCommand {
    let mut command_split: Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();
    // println!("DEBUG Split input: {:?}", command_split);
    match command_split.len() {
        0 => KshCommand {
            keyword: "".to_string(),
            args: Vec::new(),
        },
        _ => KshCommand {
            keyword: command_split.remove(0),
            args: command_split,
        },
    }
}

pub fn process_command(c: &KshCommand) -> Result<KshOutput, KshOutput> {
    match KshBuiltin::from_str(&c.keyword) {
        Ok(KshBuiltin::Echo) => builtin_echo(&c.args),
        Ok(KshBuiltin::Eval) => builtin_eval(&c.args),

        _ => Err(KshOutput {
            code: Some(1),
            stdout: String::from("").into_bytes(),
            stderr: String::from(format!("{}: Command not found", &c.keyword)).into_bytes(),
        }),
    }
}
