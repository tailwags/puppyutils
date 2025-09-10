use std::{
    io::{Write, stdin, stdout},
    process::exit,
};

use puppyutils::{Result, cli};
use xenia_utils::termios::{isatty, ttyname};

pub fn main() -> Result {
    let mut stdout = stdout();
    let stdin = stdin();

    let mut quiet = false;

    cli! {
        "tty", stdout, #error
        Short('s') | Long("silent") | Long("quiet") => quiet = true
    };

    let is_tty = isatty(&stdin);

    if quiet {
        if is_tty { exit(0) } else { exit(1) }
    }

    if is_tty {
        let name = ttyname(&stdin, Vec::new())?;
        stdout.write_all(name.as_bytes())?;
    } else {
        stdout.write_all(b"not a tty")?;
    }

    stdout.write_all(b"\n")?;
    stdout.flush()?;

    Ok(())
}
