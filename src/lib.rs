use std::{
    fmt::{Debug, Display},
    io,
};

use rustix::{fs::Mode, process::umask};

pub type Result<T = (), E = Exit> = std::result::Result<T, E>;

/// Represents the type of error
/// that caused the program to exit
/// prematurely
pub enum Exit {
    ArgError(sap::ParsingError),
    IoError(io::Error),
}

impl Debug for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgError(err) => Display::fmt(err, f),
            Self::IoError(err) => Display::fmt(err, f),
        }
    }
}

impl From<rustix::io::Errno> for Exit {
    fn from(value: rustix::io::Errno) -> Self {
        Self::IoError(value.into())
    }
}

impl From<sap::ParsingError> for Exit {
    fn from(value: sap::ParsingError) -> Self {
        Self::ArgError(value)
    }
}

impl From<io::Error> for Exit {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

#[macro_export]
macro_rules! version_text {
    ($name: literal, $authors: literal) => {
        concat!(
            $name,
            " (puppyutils) ",
            "0.0.1",
            "\ntodo: license text!", // <- todo
            "\n\n",
            $authors
        )
    };

    ($name: literal) => {
        concat!(
            $name,
            " (puppyutils) ",
            "0.0.1",
            "\ntodo: license text!", // <- todo
            "\n",
        )
    };
}

#[macro_export]
macro_rules! help_text {
    ($name: literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/", $name, ".txt"))
    };
}

/// Gets the umask of the calling process
pub fn get_umask() -> Mode {
    /*
    FIXME: Behold the umask abomination, a perfect example of UNIX's "let's make everything
    as backwards and painful as possible" design philosophy. This cursed bitmask lives in
    kernel space and can ONLY be read by WRITING TO IT because someone in 1973
    decided that having a simple getter function was too logical. So here we are, doing
    the umask shuffle: set it to 0, capture the old value, then set it back like some
    demented atomic operation that's not actually atomic. I mean who needs threads anyway,
    it's not like computers will ever be powerful enough to do more things at once,
    not for hundreds of years at the very least.

    *sigh*

    We SHOULD read from /proc/self/status like any sane program except nobody does that?
    but no - we're stuck with this fossilized turd of an API that makes you modify
    global state just to observe it. At least on anything older than 2016 which is more common
    than you'd think, sob

    I HATE UNIX
    I HATE UNIX
    I HATE UNIX
    */
    let current_umask = umask(Mode::empty());
    umask(current_umask);

    current_umask
}
