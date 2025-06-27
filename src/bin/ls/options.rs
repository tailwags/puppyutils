#![allow(dead_code, unused_variables)]

use std::{fmt::Display, num::NonZero};
#[repr(u8)]
#[derive(Copy, Clone)]
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

#[derive(Debug)]
pub(crate) enum SizeParseError<'a> {
    TooLarge(&'a [u8]),
    InvalidSuffix(&'a [u8]),
    InvalidArgument(&'a [u8]),
}

impl Display for SizeParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for SizeParseError<'_> {}

pub(crate) fn size_arg_to_multiplier(arg: &[u8]) -> Result<NonZero<u64>, SizeParseError<'_>> {
    use core::str::from_utf8_unchecked;
    let mut num_len = 0;

    while arg[num_len].is_ascii_digit() {
        num_len += 1;
    }

    if num_len == 0 {
        let err = SizeParseError::InvalidArgument(arg);
        return Err(err);
    }

    let digits = &arg[..num_len + 1];

    // SAFETY:
    //
    // The above loop guarantees that
    // the bytes in this subslice
    // represent only ASCII digits.
    let multiplier = unsafe {
        from_utf8_unchecked(digits)
            .parse::<u64>()
            .expect("infallible")
    };

    if multiplier == 0 {
        let err = SizeParseError::InvalidArgument(arg);
        return Err(err);
    }

    let (base, shift) = match &arg[num_len..] {
        b"K" | b"KiB" => (1024_u64, 1_u32),
        b"M" | b"MiB" => (1024, 10),
        b"G" | b"GiB" => (1024, 20),
        b"T" | b"TiB" => (1024, 30),
        b"P" | b"PiB" => (1024, 40),
        b"E" | b"EiB" => (1024, 50),

        b"KB" => (1024, 0),
        b"MB" => (1000, 10),
        b"GB" => (1000, 20),
        b"TB" => (1000, 30),
        b"PB" => (1000, 40),
        b"EB" => (1000, 50),

        b"" => return unsafe { Ok(NonZero::new_unchecked(multiplier)) },

        b"Z" | b"ZiB" | b"Y" | b"YiB" | b"R" | b"RiB" | b"ZB" | b"RB" | b"YB" => {
            let err = SizeParseError::TooLarge(arg);
            return Err(err);
        }

        _ => return Err(SizeParseError::InvalidSuffix(arg)),
    };

    let unit = match base.checked_shl(shift) {
        Some(val) => val,
        None => {
            let err = SizeParseError::TooLarge(arg);
            return Err(err);
        }
    };

    debug_assert!(
        multiplier != 0 && unit != 0,
        "this equation would end up as zero"
    );

    match unit.checked_mul(multiplier) {
        None => Err(SizeParseError::TooLarge(arg)),

        // SAFETY:
        //
        // The check above that `multiplier` is non-zero
        // guarantees this value is non-zero
        // the other part of the multiplication (`unit`) is
        // always guaranteed to be above 0
        Some(val) => unsafe { Ok(NonZero::new_unchecked(val)) },
    }
}
