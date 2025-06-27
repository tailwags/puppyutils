use std::{
    io::{Write, stdout},
    process::exit,
};

use puppyutils::{Result, cli};
use rustix::{
    process::{EXIT_FAILURE, EXIT_SUCCESS},
    termios::{isatty, ttyname},
};

pub fn main() -> Result {
    let mut stdout = stdout();

    let mut quiet = false;

    cli! {
        "tty", stdout, #error
        Short('s') | Long("silent") | Long("quiet") => quiet = true
    };

    if quiet {
        if isatty(stdout) {
            exit(EXIT_SUCCESS)
        } else {
            exit(EXIT_FAILURE)
        }
    }

    let name = ttyname(&stdout, Vec::new())?;

    stdout.write_all(name.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;

    Ok(())
}
