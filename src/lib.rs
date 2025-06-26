use std::{
    borrow::Cow,
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
    Custom(Cow<'static, str>),
}

impl Debug for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgError(err) => Display::fmt(err, f),
            Self::IoError(err) => Display::fmt(err, f),
            Self::Custom(err) => Display::fmt(err, f),
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

impl From<&'static str> for Exit {
    fn from(value: &'static str) -> Self {
        Self::Custom(Cow::Borrowed(value))
    }
}

#[macro_export]
macro_rules! version_text {
    ($name:literal, $authors:literal) => {
        concat!(
            $name,
            " (puppyutils) ",
            "0.0.1\n",
            "Licensed under the European Union Public Licence (EUPL) <https://eupl.eu/>"
        )
    };

    ($name:literal) => {
        concat!(
            $name,
            " (puppyutils) ",
            "0.0.1\n",
            "Licensed under the European Union Public Licence (EUPL) <https://eupl.eu/>\n"
        )
    };
}

#[macro_export]
macro_rules! help_text {
    ($name:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/", $name, ".txt"))
    };
}

/// Internal helper macro
#[macro_export]
#[doc(hidden)]
macro_rules! _cli_impl {
    ($name:literal, $stdout:ident, $loop_type:tt, $($item:pat => $matcher:expr)*) => {
        {
            let mut arg_parser = sap::Parser::from_env()?;

            $crate::_cli_impl!(arg_parser, $name, $stdout, $loop_type, $($item => $matcher)*);

            arg_parser
        }
    };

    // TODO: we can prob make a default case that uses while
    ($args:ident, $name:literal, $stdout:ident, $loop_type:tt, $($item:pat => $matcher:expr)*) => {
        $loop_type let Some(arg) = $args.forward()? {
            use std::io::Write;
            use sap::Argument::*;
            match arg {
                Long("version") => {
                    $stdout.write_all($crate::version_text!($name).as_bytes())?;
                    $stdout.flush()?;
                    std::process::exit(0);
                }
                Long("help") => {
                    $stdout.write_all($crate::help_text!($name).as_bytes())?;
                    $stdout.flush()?;
                    std::process::exit(0);
                }
                $($item => $matcher,)*
            }
        }
    };
}

#[macro_export]
macro_rules! cli {
    ($name:literal, $stdout:ident $($item:pat => $matcher:expr)*) => {
        $crate::_cli_impl!($name, $stdout, while, $($item => $matcher)*)
    };

    ($name:literal, $stdout:ident, #ignore $($item:pat => $matcher:expr)*) => {
        $crate::_cli_impl!($name, $stdout, if, $($item => $matcher)* _ => {})
    };

    ($name:literal, $stdout:ident, #fall $($item:pat => $matcher:expr)*) => {
        $crate::_cli_impl!($name, $stdout, while, $($item => $matcher)* _ => {})
    };

    ($name:literal, $stdout:ident, #error $($item:pat => $matcher:expr)*) => {
        $crate::_cli_impl!($name, $stdout, while, $($item => $matcher)* arg => return Err(arg.into_error(None).into()))
    };
}

#[macro_export]
macro_rules! cli_with_args {
    ($args:ident, $name:literal, $stdout:ident $($item:pat => $matcher:expr)*) => {
        $crate::_cli_impl!($args, $name, $stdout, while, $($item => $matcher)*)
    };

    ($args:ident, $name:literal, $stdout:ident, #ignore $($item:pat => $matcher:expr)*) => {
        $crate::_cli_impl!($args, $name, $stdout, if, $($item => $matcher)* _ => {})
    };

    ($args:ident, $name:literal, $stdout:ident, #fall $($item:pat => $matcher:expr)*) => {
        $crate::_cli_impl!($args, $name, $stdout, while, $($item => $matcher)* _ => {})
    };

    ($args:ident, $name:literal, $stdout:ident, #error $($item:pat => $matcher:expr)*) => {
        $crate::_cli_impl!($args, $name, $stdout, while, $($item => $matcher)* arg => return Err(arg.into_error(None).into()))
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

    if let Ok(status) = fs::read_to_string("/proc/self/status")
        && let Some(umask) = status
            .lines()
            .find_map(|line| RawMode::from_str_radix(line.strip_prefix("Umask:")?.trim(), 8).ok())
    {
        return Mode::from_raw_mode(umask);
    }

    // Fallback method with a possible race condition
    let current_umask = umask(Mode::empty());
    umask(current_umask);

    current_umask
}
