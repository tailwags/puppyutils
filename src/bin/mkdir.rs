use std::io::stdout;

use puppyutils::{Result, cli, get_umask};
use xenia::{Mode, mkdir};

pub fn main() -> Result {
    let mut stdout = stdout();

    let mut dirs = Vec::new();

    cli! {
        "mkdir", stdout, #error
        Value(value) => {
            dirs.push(value.into_owned()); // FIXME: I am not a fan of all this allocation but we can't easily move out of the argument parser
        }
    };

    let mode = Mode::from_bits_retain(0o777) & !get_umask();

    for dir in dirs {
        mkdir(dir, mode)?;
    }

    Ok(())
}
