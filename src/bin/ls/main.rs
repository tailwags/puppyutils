mod options;
mod settings;

use coreutils::Result;
use rustix::{
    fs::{Dir, Mode, OFlags, open},
    termios::tcgetwinsize,
};
use std::io::{BufWriter, Stdout, Write, stdout};

const VERSION: &str = coreutils::version_text!("ls");
const HELP: &str = coreutils::help_text!("ls");
const CURRENT_DIR_PATH: &str = ".";

fn main() -> Result {
    let mut out = BufWriter::new(stdout());
    let winsize = get_win_size();
    let (cfg, help_or_version) =
        settings::parse_arguments(winsize.ws_col, &mut out, HELP, VERSION)?;

    if !help_or_version {
        drop(cfg);
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

        print_all(names, out)?;
    }

    Ok(())
}

fn get_win_size() -> rustix::termios::Winsize {
    let stderr_fd = rustix::stdio::stderr();
    tcgetwinsize(stderr_fd).expect("couldn't get terminal size")
}

// FIXME: This algorithm to print out lines is incredibly simplistic
// and slightly worse than the one used in GNU's ls.
fn print_all(cols: Vec<String>, stdout: BufWriter<Stdout>) -> Result {
    const MIN_COLUMN_WIDTH: u16 = 3;

    let len = cols.len();
    let stderr_fd = rustix::stdio::stderr();
    let winsize = tcgetwinsize(stderr_fd).expect("couldn't get terminal size");

    let max_idx = ((winsize.ws_col / 3) / MIN_COLUMN_WIDTH - 1) as usize;

    let max_cols = if max_idx < len { max_idx } else { len };

    print_into_columns(cols.iter().map(String::as_str), max_cols, stdout)
}

fn print_into_columns<I>(iter: I, columns: usize, mut stdout: BufWriter<Stdout>) -> Result
where
    I: IntoIterator<Item: AsRef<str> + core::fmt::Display>,
{
    let mut counter = 0;
    for line in iter {
        if counter == columns {
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
