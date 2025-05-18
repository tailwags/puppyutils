#![allow(unused)] // FIXME: remove this
#![allow(clippy::all)] // FIXME: This is horrible but this file is a mess anyway

use coreutils::Result;
use rustix::{
    fs::{Dir, Mode, OFlags, open},
    termios::tcgetwinsize,
};
use sap::Parser;
use std::io::{BufWriter, Write, stdout};

const CURRENT_DIR_PATH: &str = ".";

#[repr(u8)]
enum FileType {
    Regular = 0,
    Directory = 1,
    BlockDevice = 2,
    SymbolicLink = 3,
    Socket = 4,
    CharacterSpecial = 5,
    Fifo = 6,
}

impl FileType {
    fn get_letter(&self) -> &'static str {
        use FileType::*;

        match self {
            Regular => "-",
            Directory => "d",
            BlockDevice => "b",
            SymbolicLink => "l",
            Socket => "s",
            CharacterSpecial => "c",
            Fifo => "p",
        }
    }
}

struct Permissions(u16);

impl Permissions {
    fn new(perms: u16) -> Option<Self> {
        if perms > 511 { None } else { Some(Self(perms)) }
    }
}

fn main() -> Result {
    let mut any_args = false;
    let mut args = Parser::from_env()?;

    while let Some(arg) = args.forward()? {
        if !any_args {
            any_args = true;
        }

        match arg {
            _ => {}
        }
    }

    if any_args {
        todo!()
    } else {
        // We received no arguments
        // therefore we can do the most "basic" action
        // just listing contents of the current directory.

        let fd = open(
            CURRENT_DIR_PATH,
            OFlags::DIRECTORY | OFlags::RDONLY,
            Mode::RUSR,
        )?;

        let dir = Dir::new(fd)?;

        // bad bad bad
        // FIXME: do not allocate
        let names = dir
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|entry| !entry.starts_with('.'))
            .collect::<Vec<_>>();

        print_all(names)?;
    }

    Ok(())
}

// FIXME: This algorithm to print out lines is incredibly simplistic
// and slightly worse than the one used in GNU's ls.
fn print_all(cols: Vec<String>) -> Result {
    const MIN_COLUMN_WIDTH: u16 = 3;

    let len = cols.len();
    let stderr_fd = rustix::stdio::stderr();
    let winsize = tcgetwinsize(stderr_fd).expect("couldn't get terminal size");

    let max_idx = ((winsize.ws_col / 3) / MIN_COLUMN_WIDTH - 1) as usize;

    let max_cols = if max_idx < len { max_idx } else { len };

    print_into_columns(cols.iter().map(String::as_str), max_cols)
}

fn print_into_columns<I>(iter: I, columns: usize) -> Result
where
    I: IntoIterator<Item: AsRef<str> + core::fmt::Display>,
{
    let mut stdout = BufWriter::new(stdout());
    let mut counter = 0;
    for line in iter {
        if counter == columns {
            print!("\n");
            stdout.write_all(b"\n")?;
            counter = 0;
        }

        if counter == columns - 1 {
            stdout.write_all(line.as_ref().as_bytes())?;
        } else {
            stdout.write_all(line.as_ref().as_bytes())?;
            stdout.write_all(b"  ")?;
        }

        counter += 1;
    }

    // fixes the shell returning a "return symbol" at the end.
    stdout.write_all(b"\n")?;
    stdout.flush()?;

    Ok(())
}
