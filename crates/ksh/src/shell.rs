use crate::repl::Repl;
use crate::{new_line, process_command, tokenize_command};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use kopy_core::vga_buffer::WRITER;
use kopy_core::{eprintln, print, println};

#[derive(Debug)]
pub struct KshCommand {
    pub keyword: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct KshOutput {
    pub(crate) code: Option<i32>,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stderr: Vec<u8>,
}

impl fmt::Display for KshCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.keyword, self.args.join(" "))
    }
}

impl Repl for KshCommand {
    type ReplResult = KshOutput;
    type ReplError = KshOutput;

    fn read() -> Self {
        let mut command = { WRITER.lock().read_line() };

        // println!("DEBUG Raw input: {:?}", command);

        tokenize_command(command.replace("$ ", ""))
    }

    fn evaluate(&self) -> Result<Self::ReplResult, Self::ReplError> {
        print!("\n");
        process_command(self)
    }

    fn print(output: Result<Self::ReplResult, Self::ReplError>) {
        match output {
            Ok(o) => {
                if !o.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8(o.stderr).unwrap())
                }
                if !o.stdout.is_empty() {
                    println!("{}", String::from_utf8(o.stdout).unwrap())
                }
            }
            Err(e) => {
                if !e.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8(e.stderr).unwrap())
                }
                if !e.stdout.is_empty() {
                    println!("{}", String::from_utf8(e.stdout).unwrap())
                }
            }
        }
    }

    fn loop_interactive() {
        Self::print(Self::evaluate(&Self::read()));
        new_line();
    }
}
