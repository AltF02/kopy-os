use crate::shell::KshOutput;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str::FromStr;

pub enum KshBuiltin {
    Echo,
    Eval,
}

impl FromStr for KshBuiltin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(KshBuiltin::Echo),
            "eval" => Ok(KshBuiltin::Eval),
            _ => Err(()),
        }
    }
}

pub fn builtin_echo(args: &Vec<String>) -> Result<KshOutput, KshOutput> {
    Ok(KshOutput {
        code: Some(0),
        stdout: String::from(args.join(" ")).into_bytes(),
        stderr: String::from("").into_bytes(),
    })
}

pub fn builtin_eval(args: &Vec<String>) -> Result<KshOutput, KshOutput> {
    Err(KshOutput {
        code: Some(1),
        stdout: String::from("").into_bytes(),
        stderr: String::from("Not yet implemented").into_bytes(),
    })
}
