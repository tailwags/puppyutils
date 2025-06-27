pub(crate) mod options;
pub(crate) mod settings;
pub(crate) mod sorting;
pub(crate) mod traverse;

use puppyutils::Result;
use rustix::termios::tcgetwinsize;
use traverse::Printer;

use std::io::stdout;

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

    let printer = Printer::new(cfg, &mut stdout);

    printer?.traverse()?;
    Ok(())
}

fn get_win_size() -> rustix::termios::Winsize {
    let stderr_fd = rustix::stdio::stderr();
    tcgetwinsize(stderr_fd).expect("couldn't get terminal size")
}
