#![allow(dead_code, unused_variables)]
#[repr(u8)]
pub(crate) enum SortOrder {
    None,
    Name,
    Size,
    Version,
    Extension,
    Directory,
    Width,
    AccessTime,
    Time,
}

#[non_exhaustive]
#[repr(u8)]
pub(crate) enum TimeStampType {
    FullIso,
    LongIso,
    Iso,
    Locale,
    // Format
}

#[repr(u8)]
pub(crate) enum QuotingStyle {
    C,
    Literal,
    Locale,
    Shell,
    ShellAlways,
    ShellEscape,
    ShellEscapeAlways,
}

#[repr(u8)]
pub(crate) enum IndicatorStyle {
    None,
    Slash,
    FileType,
    Classify,
}

#[repr(u8)]
pub(crate) enum Formatting {
    Long,
    Horizontal,
    Across,
    Commas,
    SingleCol,
}

#[repr(u8)]
pub(crate) enum When {
    Never,
    Auto,
    Always,
}

#[repr(u8)]
pub enum Dereference {
    // follow symlinks listed on the command line
    Commandline,

    // follow all symlinks in the directory
    // only if they point to a directory
    FollowAllDirs,

    // just don't do it.
    None,
}

macro_rules! from_bytes {
    ($($item: ident >> $({$($ret: tt)*} => {$($tree: tt)*})*),*) => {
        $(
            impl $item {
                /// Generates an enum from the provided bytes,
                /// returns a error-handling closure if they
                /// can't be converted to an appropriate type.
                #[inline]
                pub(crate) fn from_bytes<A>(val: A) -> Option<$item>
                where
                    A: AsRef<[u8]>,
                {
                    match val.as_ref() {
                         $($($tree)* => Some($item::$($ret)*),)*

                         _ => None

                    }
                }
            }
        )*
    };
}

from_bytes! {
    When >> {Always} => {b"always" | b"yes" | b"force"}
          {Never} => {b"never"|b"no"|b"none"}
          {Auto} => {b"auto" | b"tty" | b"if-tty"},

    Formatting >> {Long} => {b"verbose" | b"long"}
                  {Horizontal} => {b"horizontal"}
                  {Across} => {b"across"}
                  {Commas} => {b"commas"}
                  {SingleCol} => {b"single-column"},

    IndicatorStyle >> {None} => {b"none"}
                      {Slash} => {b"slash"}
                      {FileType} => {b"file-type"}
                      {Classify} => {b"classify"},

    QuotingStyle >> {Literal} => {b"literal"}
                    {Locale} => {b"locale"}
                    {Shell} => {b"shell"}
                    {ShellEscape} => {b"shell-always"}
                    {ShellEscapeAlways} => {b"shell-escape-always"}
                    {C} => {b"c"},
    SortOrder >> {None} => {b"none"}
                 {Size} => {b"size"}
                 {Time} => {b"time"}
                 {Version} => {b"version"}
                 {Extension} => {b"extension"}
                 {Width} => {b"width"}
}
