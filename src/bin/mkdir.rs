use std::io::{Write, stdout};

use coreutils::{Result, get_umask, help_text, version_text};
use rustix::fs::{Mode, mkdir};
use sap::{
    Argument::{Long, Value},
    Parser,
};

const VERSION: &str = version_text!("mkdir");
const HELP: &str = help_text!("mkdir");

fn main() -> Result {
    let mut stdout = stdout();

    let mut arg_parser = Parser::from_env()?;

    let mut dirs = Vec::new();

    while let Some(arg) = arg_parser.forward()? {
        match arg {
            Long("version") => {
                stdout.write_all(VERSION.as_bytes())?;
                stdout.flush()?;
                return Ok(());
            }
            Long("help") => {
                stdout.write_all(HELP.as_bytes())?;
                stdout.flush()?;
                return Ok(());
            }
            Value(value) => {
                dirs.push(value.to_owned()); // FIXME: I am not a fan of all this allocation but we can't easily move out of arg_parser
            }
            argument => return Err(argument.into_error(None).into()),
        }
    }

    let mode = Mode::from_raw_mode(0o777) & !get_umask();

    for dir in dirs {
        mkdir(dir, mode)?;
    }

    Ok(())
}
