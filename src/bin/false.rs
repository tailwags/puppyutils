#![feature(exitcode_exit_method)]

use std::{
    io::{Write, stdout},
    os::unix::ffi::OsStrExt,
    process::ExitCode,
};

use coreutils::Result;

fn main() -> Result {
    let mut args = std::env::args_os();

    if args.len() == 2 {
        args.next();
        if let Some(arg) = args.next() {
            match arg.as_bytes() {
                b"--version" => {
                    stdout().write_all(b"puppyutils 0.0.1\n")?;
                    return Ok(());
                }
                b"--help" => {
                    stdout().write_all(b"Exit with a status code indicating failure.\n")?;
                    return Ok(());
                }
                _ => {}
            }
        }
    }

    ExitCode::FAILURE.exit_process()
}
