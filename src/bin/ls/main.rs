pub(crate) mod options;
pub(crate) mod settings;
pub(crate) mod sorting;
pub(crate) mod traverse;

use traverse::Printer;

use puppyutils::Result;
use rustix::{
    fs::{Dir, Mode, OFlags, open},
    termios::tcgetwinsize,
};
use std::io::{self, BufWriter, stdout};

const CURRENT_DIR_PATH: &str = ".";

pub fn main() -> Result {
    let mut stdout = stdout();
    let winsize = get_win_size();
    let cfg = settings::parse_arguments(winsize.ws_col, &mut stdout)?;

    // let fd = open(
    //     cfg.directory(),
    //     OFlags::DIRECTORY | OFlags::RDONLY,
    //     Mode::RUSR,
    // )?;

    // let dir = Dir::new(fd)?;

    // bad bad bad
    // FIXME: do not allocate
    // let names = dir
    //     .filter_map(Result::ok)
    //     .map(|entry| entry.file_name().to_string_lossy().into_owned())
    //     .filter(|entry| !entry.starts_with('.'))
    //     .collect::<Vec<_>>();

    // let mut stdout = BufWriter::new(stdout);

    // print_all(names, &mut stdout)?;

    let mut printer = Printer::new(cfg, &mut stdout);

    printer?.traverse()?;
    Ok(())
}

fn get_win_size() -> rustix::termios::Winsize {
    let stderr_fd = rustix::stdio::stderr();
    tcgetwinsize(stderr_fd).expect("couldn't get terminal size")
}

// FIXME: This algorithm to print out lines is incredibly simplistic
// and slightly worse than the one used in GNU's ls.
fn print_all<O: io::Write>(cols: Vec<String>, stdout: &mut O) -> Result {
    const MIN_COLUMN_WIDTH: u16 = 3;

    let len = cols.len();
    let stderr_fd = rustix::stdio::stderr();
    let winsize = tcgetwinsize(stderr_fd).expect("couldn't get terminal size");

    let max_idx = ((winsize.ws_col / 3) / MIN_COLUMN_WIDTH - 1) as usize;

    let max_cols = if max_idx < len { max_idx } else { len };

    print_into_columns(cols.iter().map(String::as_str), max_cols, stdout)
}

fn print_into_columns<I, O: io::Write>(iter: I, columns: usize, stdout: &mut O) -> Result
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
