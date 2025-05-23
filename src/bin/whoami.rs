use std::io::{Write, stderr, stdout};

use acumen::getpwuid;
use coreutils::{Exit, Result, help_text, version_text};
use rustix::process::geteuid;
use sap::{Argument::Long, Parser};

const VERSION: &str = version_text!("whoami");
const HELP: &str = help_text!("whoami");

const CANNOT_FIND_UID: &[u8] = b"cannot find name for user ID: ";

fn main() -> Result {
    let mut stdout = stdout();
    let mut arg_parser = Parser::from_env()?;

    if let Some(arg) = arg_parser.forward()? {
        match arg {
            Long("version") => stdout.write_all(VERSION.as_bytes())?,
            Long("help") => stdout.write_all(HELP.as_bytes())?,
            invalid => return Err(Exit::ArgError(invalid.into_error(None))),
        }

        stdout.flush()?;

        return Ok(());
    }

    let uid = geteuid();

    if let Some(passwd) = getpwuid(geteuid()) {
        stdout.write_all(passwd.name.as_bytes())?;
        stdout.write_all(b"\n")?;

        stdout.flush()?;
    } else {
        let mut stderr = stderr();

        stderr.write_all(CANNOT_FIND_UID)?;
        stderr.write_all(itoa::Buffer::new().format(uid.as_raw()).as_bytes())?;
        stderr.write_all(b"\n")?;

        stderr.flush()?;
    }

    Ok(())
}
