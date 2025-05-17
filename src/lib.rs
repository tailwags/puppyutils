use std::{fmt::Debug, io, process::ExitCode};

pub type Result<T = (), E = Exit> = std::result::Result<T, E>;

/// Represents the type of error
/// that caused the program to exit
/// prematurely
pub enum Exit {
    ArgError(sap::ParsingError),
    IoError(io::Error),
    Code(ExitCode),
}

impl Debug for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgError(err) => err.fmt(f),
            Self::IoError(err) => err.fmt(f),
            Self::Code(code) => code.fmt(f),
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

impl From<ExitCode> for Exit {
    fn from(code: ExitCode) -> Self {
        Self::Code(code)
    }
}

impl From<()> for Exit {
    fn from(_: ()) -> Self {
        Self::Code(ExitCode::SUCCESS)
    }
}
