use std::{
    borrow::Cow,
    io::{BufWriter, Write, stdout},
    os::unix::ffi::OsStringExt,
};

const VERSION: &[u8] = coreutils::version_text!("yes").as_bytes();
const HELP: &[u8] = b"Usage: yes [STRING]...\n"; // TODO: properly generate this string

use coreutils::{Exit, Result};
use sap::{
    Argument::{Long, Short, Value},
    Parser,
};

fn main() -> Result {
    let mut arg_parser = Parser::from_env()?;

    // No point in locking stdout since we only use it once in this program
    let mut stdout = stdout();

    let mut buffer: Option<Vec<u8>> = None;

    while let Some(arg) = arg_parser.forward()? {
        match arg {
            Long("version") => {
                stdout.write_all(VERSION)?;
                stdout.flush()?;

                return Ok(());
            }
            Long("help") => {
                stdout.write_all(HELP)?;
                stdout.flush()?;

                return Ok(());
            }
            Value(value) => {
                extend_buffer(&mut buffer, value.as_bytes().to_vec());
            }
            Long(_) | Short(_) => return Err(Exit::ArgError(arg.into_error(None))),
        }
    }

    arg_parser
        .into_inner()
        .for_each(|arg| extend_buffer(&mut buffer, arg.into_vec()));

    let output: Cow<'_, [u8]> = if let Some(mut buffer) = buffer {
        buffer.push(b'\n');
        Cow::Owned(buffer)
    } else {
        Cow::Borrowed(b"y\n")
    };

    // Write everything to stdout, BufWriter will handle the buffering
    let mut stdout = BufWriter::new(stdout);

    loop {
        stdout.write_all(&output)?;
    }
}

#[inline]
fn extend_buffer(buffer: &mut Option<Vec<u8>>, mut arg: Vec<u8>) {
    if let Some(buffer) = buffer {
        buffer.push(b' '); // Manually put the space
        buffer.append(&mut arg);
    } else {
        *buffer = Some(arg)
    }
}
