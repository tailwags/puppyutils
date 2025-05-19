use std::{
    borrow::Cow,
    ffi::OsString,
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

    let mut first_value: Option<Vec<u8>> = None;

    if let Some(arg) = arg_parser.forward()? {
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
            Long(_) | Short(_) => return Err(Exit::ArgError(arg.into_error(None))),
            Value(value) => first_value = Some(value.as_bytes().to_vec()),
        }
    }

    let mut args = arg_parser.into_inner();

    if first_value.is_none() {
        first_value = args.next().map(OsString::into_vec);
    }

    // We prepare the output so it doesn't need to go through the formatting each time
    let output: Cow<'_, [u8]> = if let Some(mut first) = first_value {
        for arg in args {
            first.push(b' '); // Manually put the space
            first.append(&mut arg.into_vec()); // Append will move the data efficiently from the other vector
        }

        first.push(b'\n');

        Cow::Owned(first)
    } else {
        Cow::Borrowed(b"y\n") // If there are no args we can just hardcode it and avoid allocation
    };

    // Write everything to stdout, BufWriter will handle the buffering
    let mut stdout = BufWriter::new(stdout);

    loop {
        stdout.write_all(&output)?;
    }
}
