use std::{
    io::{Write, stdout},
    os::unix::ffi::OsStrExt,
};

use coreutils::{Result, help_text, version_text};

const VERSION: &str = version_text!("true");
const HELP: &str = help_text!("true");

fn main() -> Result {
    let mut args = std::env::args_os();

    if args.len() == 2 {
        args.next();
        if let Some(arg) = args.next() {
            match arg.as_bytes() {
                b"--version" => {
                    stdout().write_all(VERSION.as_bytes())?;
                    return Ok(());
                }
                b"--help" => {
                    stdout().write_all(HELP.as_bytes())?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }

    Ok(())
}
