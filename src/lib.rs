use std::{
    fmt::{Debug, Display},
    io,
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
        include_str!(concat!("../../docs/", $name, ".txt"))
    };

    (Deep, $name: literal) => {
        include_str!(concat!("../../../docs/", $name, ".txt"))
    };
}
