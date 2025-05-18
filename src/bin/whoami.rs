use std::io::{BufWriter, Write, stderr, stdout};

use acumen::getpwuid;
use coreutils::{Exit, Result};
use rustix::process::geteuid;
use sap::{Argument::Long, Parser};

const VERSION: &str = coreutils::version_text!("whoami");

const HELP: &[u8] =
    b"Usage: whoami\nPrint the user name associated with the current effective user ID.\n";

const CANNOT_FIND_UID: &[u8] = b"cannot find name for user ID: ";

fn main() -> Result {
    let mut stdout = BufWriter::new(stdout());
    let mut arg_parser = Parser::from_env()?;

    if let Some(arg) = arg_parser.forward()? {
        match arg {
            Long("version") => {
                stdout.write_all(VERSION.as_bytes())?;
                stdout.flush()?;
                return Ok(());
            }
            Long("help") => {
                stdout.write_all(HELP)?;
                stdout.flush()?;
                return Ok(());
            }
            invalid => return Err(Exit::ArgError(invalid.into_error(None))),
        }
    }

    let uid = geteuid();

    if let Some(passwd) = getpwuid(uid) {
        stdout.write_all(passwd.name.as_bytes())?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;
    } else {
        let mut err = BufWriter::new(stderr());
        err.write_all(CANNOT_FIND_UID)?;

        // uhh...
        // FIXME: it's weird
        err.write_all(uid.as_raw().to_string().as_bytes())?;
        err.write_all(b"\n")?;
        err.flush()?;
    }

    Ok(())
}
