#![allow(dead_code)]
use super::options::*;
use puppyutils::{Result, cli_with_args};
use sap::Parser;
use std::io;
use std::num::NonZero;

const CURRENT_DIR_PATH: &str = ".";
const DEFAULT_BLOCK_SIZE: usize = 512;

fn needs_an_argument() -> ! {
    todo!()
}

fn invalid_argument() -> ! {
    todo!()
}

bitflags::bitflags! {
    #[rustfmt::skip]
    pub(crate) struct LsFlags: u32 {
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
        const HIDE_CONTROL_CHARS =       1 << 28; // -q --hide-control-chars --show-control-chars
        const QUOTE_ENTRIES =            1 << 29; // -Q --quote-name
        const DIRECTORIES_FIRST =        1 << 30;

        /// refers to the `-c` option
        /// as it needs to be evaluated last.
        const LOWERCASE_C =              1 << 31;
    }
}

pub(crate) fn parse_arguments<O: io::Write>(width: u16, out: &mut O) -> Result<LsConfig> {
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
        width,
        size_unit: None,
    };

    cli_with_args! {
        args, "ls", out
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
            // use crate::options::SizeParseError;

            if let Some(arg) = args.value() {
                match size_arg_to_multiplier(arg.as_bytes()) {
                    Err(err) => match err {
                        SizeParseError::TooLarge(_arg) => {
                            todo!()
                        }

                        SizeParseError::InvalidSuffix(_arg) => {
                            todo!()
                        }

                        SizeParseError::InvalidArgument(_arg) => {
                            todo!()
                        }

                    }

                    Ok(val) => settings.size_unit = Some(val)
                }
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

        Short('d') | Long("directory") => settings.flags |= LsFlags::LIST_DIRECTORIES

        Short('D') | Long("dired") => settings.flags |= LsFlags::EMACS_DIRED_MODE

        Short('f') => {
            settings.flags &= !LsFlags::SORT_ENTRIES;
            settings.flags |= LsFlags::NOT_IGNORE_DOTS;
            settings.flags |= LsFlags::DIRECTORIES_FIRST;
            settings.color = When::Never;
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

        Short('N') | Long("literal") => {
            settings.flags |= LsFlags::LITERAL_NAMES;
        }

        Short('o') => {
            settings.flags |= LsFlags::NO_GROUPS_LISTED;
            settings.format = Formatting::Long;
        }

        Short('p') => {
            settings.indicator = IndicatorStyle::Slash;
        }

        Short('q') | Long("hide-control-chars") => {
            settings.flags |= LsFlags::HIDE_CONTROL_CHARS;
        }

        Long("show-control-chars") => {
            settings.flags &= !LsFlags::HIDE_CONTROL_CHARS;
        }

        Short('Q') | Long("quote-name") => {
            settings.flags |= LsFlags::QUOTE_ENTRIES;
        }

        Long("quoting-style") => {
            settings.quoting = match args.value() {
                None => needs_an_argument(),

                Some(val) => match val.as_str() {
                    "literal" => QuotingStyle::Literal,
                    "locale" => QuotingStyle::Locale,
                    "shell" => QuotingStyle::Shell,
                    "shell-always" => QuotingStyle::ShellAlways,
                    "shell-escape" => QuotingStyle::ShellEscape,
                    "shell-escape-always" => QuotingStyle::ShellEscapeAlways,
                    "c" => QuotingStyle::C,

                    _ => invalid_argument(),
                },
            }
        }

        Short('r') | Long("reverse") => {
            settings.flags |= LsFlags::REVERSE_SORT;
        }

        Short('R') | Long("recursive") => {
            settings.flags |= LsFlags::RECURSIVE;
        }

        Short('s') | Long("size") => {
            settings.flags |= LsFlags::PRINT_ALLOCATED_SIZE;
        }

        Short('S') => {
            settings.order = SortOrder::Size;
        }

        Long("sort") => {
            settings.order = if let Some(val) = args.value() {
                if let Some(ret) = SortOrder::from_bytes(val.as_str()) {
                    ret
                } else {
                    invalid_argument()
                }
            } else {
                needs_an_argument();
            }
        }

        Long("time") => {
            // todo
        }

        Long("time-style") => {
            // todo
        }

        Short('t') => {
            // todo
        }

        Short('T') | Long("tabsize") => {
            // todo
        }

        Short('u') => {
            match (&settings.format, &settings.order) {
                (Formatting::Long, SortOrder::AccessTime) => {
                    // i don't know
                }

                (Formatting::Long, _) => {
                    settings.order = SortOrder::Name;

                    // "show access time"
                }

                _ => {
                    settings.order = SortOrder::AccessTime;
                }
            }
        }

        Short('U') => {
            settings.order = SortOrder::None;
            settings.flags |= LsFlags::DIRECTORIES_FIRST;
        }

        Short('v') => {
            // what does "natural sort of version numbers" mean...
        }

        Short('w') | Long("width") => {
            // limit width via another option...
        }

        Short('x') => {
            // needs a new format mode...
        }

        Short('X') => {
            settings.order = SortOrder::Extension;
        }

        Short('Z') | Long("context") => {
            // i need to learn what is a security context..
        }

        Long("zero") => {
            settings.flags |= LsFlags::END_WITH_NUL;
        }

        Short('1') => {
            settings.flags |= LsFlags::ONE_FILE_PER_LINE;
        }

        _ => {
            // todo!
        }
    }

    Ok(settings)
}

pub(crate) struct LsConfig {
    // order by which the entries will be sorted.
    pub order: SortOrder,

    // time of timestamp used by ls
    pub time_ty: TimeStampType,

    // settings that could be contained in bitflags.
    pub flags: LsFlags,

    // quoting style for names
    pub quoting: QuotingStyle,

    // indicator style to append to entry names.
    pub indicator: IndicatorStyle,

    // specifies how and which symlinks
    // should be dereferenced
    pub deref: Dereference,

    // related to --color.
    pub color: When,

    // related to --hyperlink
    pub hyperlink_file_names: When,

    // related to --classify and -F
    pub classify_files: When,

    // directory to search through.
    pub dir: Option<String>,

    // block size
    pub blk_size: usize,

    // formatting used
    pub format: Formatting,

    // line width.
    pub width: u16,

    // size from `--block-size`
    pub size_unit: Option<NonZero<u64>>,
}

impl LsConfig {
    pub(crate) fn directory(&self) -> &str {
        self.dir.as_deref().unwrap_or(CURRENT_DIR_PATH)
    }
}
