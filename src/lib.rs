use std::{
    fmt::{Debug, Display},
    fs, io,
};

use rustix::{
    fs::{Mode, RawMode},
    process::umask,
};

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
    Historical note: The traditional umask() syscall is a perfect example of UNIX's "let's make
    everything as backwards and painful as possible" design philosophy. This cursed bitmask
    lives in kernel space and could ONLY be read by WRITING TO IT because someone in 1973
    decided that having a simple getter function was too logical. The classic approach required
    the umask shuffle: set it to 0, capture the old value, then set it back like some
    demented atomic operation that's not actually atomic.

    Fortunately, modern Linux (kernel 4.7+, 2016) added umask to /proc/self/status, so we
    can read it cleanly without the race condition. We fall back to the traditional method
    for older systems or non-Linux platforms.

    The old way was truly cursed - modifying global state just to observe it. At least
    we've mostly escaped that particular circle of UNIX hell on modern systems.
    */

    if let Ok(status) = fs::read_to_string("/proc/self/status") {
        if let Some(umask) = status
            .lines()
            .find_map(|line| RawMode::from_str_radix(line.strip_prefix("Umask:")?.trim(), 8).ok())
        {
            return Mode::from_raw_mode(umask);
        }
    }

    // Fallback method with a possible race condition
    let current_umask = umask(Mode::empty());
    umask(current_umask);

    current_umask
}
