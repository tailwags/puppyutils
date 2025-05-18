use std::{
    borrow::Cow,
    env,
    io::{BufWriter, Write, stdout},
    os::unix::ffi::OsStringExt,
};

const VERSION: &[u8] = coreutils::version_text!("yes").as_bytes();
const HELP: &[u8] = b"Usage: yes [STRING]...\n"; // TODO: properly generate this string

use coreutils::{Exit, Result};
use sap::{
    Argument::{Long, Short},
    Parser,
};

fn main() -> Result {
    let mut arg_parser = Parser::from_env()?;

    // No point in locking stdout since we only use it once in this program
    let mut stdout = stdout();

    if let Some(arg) = arg_parser.forward()? {
        match arg {
            Long("version") => stdout.write_all(VERSION)?,
            Long("help") => stdout.write_all(HELP)?,
            Long(_) | Short(_) => return Err(Exit::ArgError(arg.into_error(None))),
            _ => {}
        }

        stdout.flush()?;

        return Ok(());
    }

    // We can easily avoid the overhead of utf-8 since this is unix anyway
    let mut args = env::args_os();
    args.next(); // Calling next once is actually more efficient than using skip, we need this to skip the program name itself

    // We prepare the output so it doesn't need to go through the formatting each time
    let output: Cow<'_, [u8]> = if let Some(msg) = args.next() {
        let mut first = msg.into_vec(); // The first argument is always ready and we have a preallocated vec, we can just reuse it

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
