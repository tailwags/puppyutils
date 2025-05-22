#![allow(unused)] // FIXME: remove this
#![warn(clippy::all)] // FIXME: This is horrible but this file is a mess anyway

use bitflags::{Flags, bitflags};
use coreutils::Result;
use rustix::{
    fs::{Dir, Mode, OFlags, open, setxattr},
    termios::tcgetwinsize,
};
use sap::{
    Argument::{Long, Short, Value},
    Parser,
};
use std::io::{BufWriter, Write, stdout};

const DEFAULT_BLOCK_SIZE: usize = 512;

bitflags! {
    #[rustfmt::skip]
    struct LsFlags: u32 {
        const NOT_IGNORE_DOTS =          1 << 0;  // -a --all
        const IGNORE_DOTS_EXCEPT_DIRS =  1 << 1;  // -A --almost-all
        const PRINT_AUTHOR =             1 << 2;  // --author
        const LIST_DIRECTORIES =         1 << 3;  // -d -- directory
        const C_STYLE_ESCAPED =          1 << 4;  // -b --escape
        const IGNORE_TILDE_ENTRIES =     1 << 5;  // -B --ignore-backups
        const LIST_BY_COLUMNS =          1 << 6;  // -C
        const EMACS_DIRED_MODE =         1 << 7;  // -D --dired
        const SORT_ENTRIES =             1 << 8;  // partially -f (can disable it) and -U
        const NO_OWNER_LISTED =          1 << 9;  // related to -g
        const NO_GROUPS_LISTED =         1 << 10; // -G --no-group
        const HUMAN_READABLE_SIZES =     1 << 11; // -h --human-readable
        const SI_SIZES =                 1 << 12; // --si
        const PRINT_INODE_INDEXES =      1 << 13; // -i --inode
        const KB_BLOCKS =                1 << 14; // -k --kibibytes
        const GROUP_DIRS_FIRST =         1 << 15; // --group-directories-first
        const DEREF_SYMLINKS =           1 << 16; // -L --dereference
        const COMMA_SEP_LIST =           1 << 17; // -m
        const NUMERIC_IDS =              1 << 18; // -n --numeric-uid-gid
        const LITERAL_NAMES =            1 << 19; // -N --literal
        const REVERSE_SORT =             1 << 20; // -r, --reverse
        const RECURSIVE =                1 << 21; // -R --recursive
        const PRINT_ALLOCATED_SIZE =     1 << 22; // -s --size
        const SORT_BY_VERSION_NUMBER =   1 << 23; // -v
        const LIST_BY_LINES =            1 << 24; // -x
        const PRINT_SECURIT_CTXT =       1 << 25; // -Z --context
        const END_WITH_NUL =             1 << 26; // --zero
        const ONE_FILE_PER_LINE =        1 << 27; // -1

        /// refers to the `-c` option
        /// as it needs to be evaluated last.
        const LOWERCASE_C =              1 << 28;
    }
}

fn parse_arguments() -> Result<(LsConfig, bool)> {
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
        format: Formatting::Horizontal, // default seems to be horizontal
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
                    needs_an_argument();
                }
            }

            Short('B') | Long("ignore-backups") => {
                settings.flags |= LsFlags::IGNORE_TILDE_ENTRIES;
            }

            Short('c') => {
                settings.flags |= LsFlags::LOWERCASE_C;
            }

            Short('C') => {
                settings.flags |= LsFlags::LIST_BY_COLUMNS;
            }

            Long("color") => {
                if let Some(value) = args.value() {
                    match When::from_bytes(&value) {
                        None => invalid_argument(),
                        Some(color) => settings.color = color,
                    }
                } else {
                    needs_an_argument()
                }
            }

            Short('d') | Long("directory") => settings.flags |= LsFlags::LIST_DIRECTORIES,

            Short('D') | Long("dired") => settings.flags |= LsFlags::EMACS_DIRED_MODE,

            Short('f') => {
                settings.flags &= !LsFlags::SORT_ENTRIES;
                settings.flags |= LsFlags::NOT_IGNORE_DOTS;

                settings.color = When::Never;
                settings.order = SortOrder::Directory;
            }

            Short('F') | Long("classify") => {
                settings.classify_files = match args.value() {
                    None => needs_an_argument(),
                    Some(val) => match When::from_bytes(&val) {
                        None => invalid_argument(),
                        Some(str) => str,
                    },
                };
            }

            Long("file-type") => {
                settings.classify_files = match args.value() {
                    None => needs_an_argument(),
                    Some(val) => match When::from_bytes(&val) {
                        None => invalid_argument(),
                        Some(str) => str,
                    },
                };

                // this should make so '*' doesn't appear
            }

            Long("format") => {
                settings.format = match args.value() {
                    None => needs_an_argument(),
                    Some(val) => match Formatting::from_bytes(&val) {
                        None => invalid_argument(),

                        Some(x) => x,
                    },
                };
            }

            Long("full-time") => {
                settings.format = Formatting::Long;
                settings.time_ty = TimeStampType::FullIso;
            }

            Short('g') => {
                settings.flags |= LsFlags::NO_OWNER_LISTED;
                settings.format = Formatting::Long;
            }

            Long("group-directories-first") => {
                settings.flags |= LsFlags::GROUP_DIRS_FIRST;
            }

            Short('G') | Long("no-group") => {
                settings.flags |= LsFlags::NO_GROUPS_LISTED;
            }

            Short('h') | Long("human-readable") => {
                settings.flags |= LsFlags::HUMAN_READABLE_SIZES;
            }

            Long("si") => {
                settings.flags |= LsFlags::SI_SIZES;
            }

            Short('H') | Long("dereference-command-line") => {
                // not sure how to handle this
                // requires some testing.
            }

            Long("derefence-command-line-symlinks") => {
                // above
            }

            Long("hide") => {
                todo!("--hide")
            }

            Long("hyperlink") => {
                settings.hyperlink_file_names = match args.value() {
                    None => needs_an_argument(),
                    Some(val) => match When::from_bytes(&val) {
                        None => invalid_argument(),

                        Some(x) => x,
                    },
                };
            }

            Long("indicator-style") => {
                settings.indicator = match args.value() {
                    None => needs_an_argument(),
                    Some(val) => match IndicatorStyle::from_bytes(&val) {
                        None => invalid_argument(),

                        Some(x) => x,
                    },
                };
            }

            Short('i') | Long("inode") => {
                settings.flags |= LsFlags::PRINT_INODE_INDEXES;
            }

            Short('I') | Long("ignore") => {
                // todo
                todo!()
            }

            Short('l') => {
                settings.format = Formatting::Long;
            }

            Short('L') | Long("dereference") => {
                settings.flags |= LsFlags::DEREF_SYMLINKS;
            }

            Short('m') => {
                settings.flags |= LsFlags::COMMA_SEP_LIST;
            }

            Short('n') | Long("numeric-uid-gid") => {
                settings.flags |= LsFlags::NUMERIC_IDS;
            }
            _ => {
                // todo!
            }
        }
    }

    Ok((settings, any_args))
}

enum SortOrder {
    None,
    Name,
    Size,
    Version,
    Extension,
    Directory,
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
impl IndicatorStyle {
    #[inline]
    fn from_bytes<A>(val: A) -> Option<Self>
    where
        A: AsRef<[u8]>,
    {
        match val.as_ref() {
            b"none" => Some(Self::None),
            b"slash" => Some(Self::Slash),
            b"file-type" => Some(Self::FileType),
            b"classify" => Some(Self::Classify),

            _ => None,
        }
    }
}

#[repr(u8)]
enum Formatting {
    Long,
    Horizontal,
    Across,
    Commas,
    SingleCol,
}

impl Formatting {
    #[inline]
    fn from_bytes<A>(val: A) -> Option<Self>
    where
        A: AsRef<[u8]>,
    {
        match val.as_ref() {
            b"verbose" | b"long" => Some(Self::Long),
            b"horizontal" => Some(Self::Horizontal),
            b"across" => Some(Self::Across),
            b"commas" => Some(Self::Commas),
            b"single-column" => Some(Self::Commas),

            _ => None,
        }
    }
}

#[repr(u8)]
enum When {
    Never,
    Auto,
    Always,
}

impl When {
    #[inline]
    fn from_bytes<A>(val: A) -> Option<Self>
    where
        A: AsRef<[u8]>,
    {
        match val.as_ref() {
            b"always" | b"yes" | b"force" => Some(Self::Always),
            b"never" | b"no" | b"none" => Some(Self::Never),
            b"auto" | b"tty" | b"if-tty" => Some(Self::Auto),

            _ => None,
        }
    }
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

    // formatting used
    format: Formatting,

    // line width.
    width: u16,
}

const CURRENT_DIR_PATH: &str = ".";

fn main() -> Result {
    println!("size_of::<LsConfig>() => {}", size_of::<LsConfig>());

    let (settings, any_args) = parse_arguments()?;

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

fn needs_an_argument() -> ! {
    todo!()
}

fn invalid_argument() -> ! {
    todo!()
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
