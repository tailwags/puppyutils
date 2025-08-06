#![allow(unused_attributes)]
#![feature(can_vector)]

use std::{
    borrow::Cow,
    io::{BufWriter, Write},
    os::fd::BorrowedFd,
};

use puppyutils::{Result, cli};
use rustix::{
    io::{write, writev},
    stdio::stdout,
};

#[repr(transparent)]
struct UnbufStdout<'a> {
    raw: BorrowedFd<'a>,
}

impl UnbufStdout<'_> {
    pub fn new() -> Self {
        Self { raw: stdout() }
    }
}

impl Write for UnbufStdout<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        write(self.raw, buf).map_err(std::io::Error::from)
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        writev(self.raw, bufs).map_err(std::io::Error::from)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        true
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn main() -> Result {
    let mut stdout = UnbufStdout::new();

    let mut buffer: Option<String> = None;

    cli! {
        "yes", stdout, #error
        Value(value) => {
             if let Some(ref mut buffer) = buffer {
                buffer.push(' '); // Manually put the space
                buffer.push_str(value.as_ref());
            } else {
                buffer = Some(value.into_owned())
            }
        }
    };

    let output = if let Some(mut buffer) = buffer {
        buffer.push('\n');
        Cow::Owned(buffer)
    } else {
        Cow::Borrowed("y\n")
    };

    // Write everything to stdout, BufWriter will handle the buffering
    let mut stdout = BufWriter::new(stdout);

    loop {
        stdout.write_all(output.as_bytes())?;
    }
}
