#![allow(unused)] // FIXME: remove this
#![warn(clippy::all)] // FIXME: This is horrible but this file is a mess anyway

use bitflags::{Flags, bitflags};
use coreutils::Result;
use rustix::{
    fs::{Dir, Mode, OFlags, open},
    termios::tcgetwinsize,
};
use sap::{
    Argument::{Long, Short, Value},
    Parser,
};
use std::io::{BufWriter, Write, stdout};

const DEFAULT_BLOCK_SIZE: usize = 512;

bitflags! {
    struct LsFlags: u32 {
        const NOT_IGNORE_DOTS = 1 << 0; // -a --all
        const IGNORE_DOTS_EXCEPT_DIRS = 1 << 1; // -A --almost-all
        const PRINT_AUTHOR = 1 << 2; // --author
        const LIST_DIRECTORIES = 1 << 3; // -d -- directory
        const C_STYLE_ESCAPED = 1 << 4; // -b --escape
        const IGNORE_TILDE_ENTRIES = 1 << 5; // -B --ignore-backups
        const LIST_BY_COLUMNS = 1 << 6; // -C
        const EMACS_DIRED_MODE = 1 << 7; // -D --dired
        const SORT_ENTRIES = 1 << 8; // partially -f (can disable it) and -U
        const NO_OWNER_LISTED = 1 << 9; // related to -g
        const NO_GROUPS_LISTED = 1 << 10; // -G --no-group
        const HUMAN_READABLE_SIZES = 1 << 11; // -h --human-readable
        const SI_SIZES = 1 << 12; // --si

        // maybe -H will be here.

        const PRINT_INODE_INDEXES = 1 << 13; // -i --inode
        const KB_BLOCKS = 1 << 14; // -k --kibibytes
        const LONG_LISTING = 1 << 15; // -l
        const DEREF_SYMLINKS = 1 << 16; // -L --dereference
        const COMMA_SEP_LIST = 1 << 17; // -m
        const NUMERIC_IDS = 1 << 18; // -n --numeric-uid-gid
        const LITERAL_NAMES = 1 << 19; // -N --literal
        const REVERSE_SORT = 1 << 20; // -r, --reverse
        const RECURSIVE = 1 << 21; // -R --recursive
        const PRINT_ALLOCATED_SIZE = 1 << 22; // -s --size
        const SORT_BY_VERSION_NUMBER = 1 << 23; // -v
        const LIST_BY_LINES = 1 << 24; // -x
        const PRINT_SECURIT_CTXT = 1 << 25; // -Z --context
        const END_WITH_NUL = 1 << 26; // --zero
        const ONE_FILE_PER_LINE = 1 << 27; // -1
    }
}

enum SortOrder {
    None,
    Name,
    Size,
    Version,
    Extension,
    Width,
    AccessTime,
    Time(TimeStampType),
}

#[non_exhaustive]
#[repr(u8)]
enum TimeStampType {
    FullIso,
    LongIso,
    Iso,
    Locale,
    // Format
}

#[repr(u8)]
enum QuotingStyle {
    Literal,
    Locale,
    Shell,
    ShellAlways,
    ShellEscape,
    ShellEscapeAlways,
}

#[repr(u8)]
enum IndicatorStyle {
    None,
    Slash,
    FileType,
    Classify,
}

#[repr(u8)]
enum When {
    Never,
    Auto,
    Always,
}

#[repr(u8)]
enum Dereference {
    // follow symlinks listed on the command line
    Commandline,

    // follow all symlinks in the directory
    // only if they point to a directory
    FollowAllDirs,

    // just don't do it.
    None,
}

struct LsConfig {
    // order by which the entries will be sorted.
    order: SortOrder,

    // time of timestamp used by ls
    time_ty: TimeStampType,

    // settings that could be contained in bitflags.
    flags: LsFlags,

    // quoting style for names
    quoting: QuotingStyle,

    // indicator style to append to entry names.
    indicator: IndicatorStyle,

    // specifies how and which symlinks
    // should be dereferenced
    deref: Dereference,

    // related to --color.
    color: When,

    // related to --hyperlink
    hyperlink_file_names: When,

    // related to --classify and -F
    classify_files: When,

    // directory to search through.
    dir: Option<String>,

    // block size
    blk_size: usize,

    // line width.
    width: u16,
}

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
    println!("size_of::<LsConfig>() => {}", size_of::<LsConfig>());
    let mut any_args = false;
    let mut args = Parser::from_env()?;

    let mut settings = LsConfig {
        flags: LsFlags::empty(),
        order: SortOrder::Name,
        time_ty: TimeStampType::Locale,
        quoting: QuotingStyle::Literal,
        indicator: IndicatorStyle::None,
        deref: Dereference::None,
        color: When::Always,
        hyperlink_file_names: When::Always,
        classify_files: When::Always,
        dir: None,
        blk_size: DEFAULT_BLOCK_SIZE,
        width: get_win_size().ws_col,
    };

    while let Some(arg) = args.forward()? {
        if !any_args {
            any_args = true;
        }

        match arg {
            Short('a') | Long("all") => {
                settings.flags |= LsFlags::NOT_IGNORE_DOTS;
            }

            Short('A') | Long("almost-all") => {
                settings.flags |= LsFlags::IGNORE_DOTS_EXCEPT_DIRS;
            }

            Long("author") => {
                settings.flags |= LsFlags::PRINT_AUTHOR;
            }

            Short('b') | Long("escape") => {
                settings.flags |= LsFlags::C_STYLE_ESCAPED;
            }

            Long("block-size") => {
                if let Some(arg) = args.value() {
                    // do the block-size
                } else {
                    // error out
                }
            }

            _ => {
                // todo!
            }
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

fn get_win_size() -> rustix::termios::Winsize {
    let stderr_fd = rustix::stdio::stderr();
    tcgetwinsize(stderr_fd).expect("couldn't get terminal size")
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
