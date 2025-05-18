use std::io::{Write, stdout};

use acumen::getpwuid;
use coreutils::{Exit, Result};
use rustix::process::geteuid;
use sap::{Argument::Long, Parser};

fn main() -> Result {
    let mut arg_parser = Parser::from_env()?;

    if let Some(arg) = arg_parser.forward()? {
        match arg {
            Long("version") => {
                println!("puppyutils 0.0.1"); // TODO: properly generate this string
                return Ok(());
            }
            Long("help") => {
                println!(
                    "Usage: whoami\nPrint the user name associated with the current effective user ID."
                );
                return Ok(());
            }
            invalid => return Err(Exit::ArgError(invalid.into_error(None))),
        }
    }

    let uid = geteuid();
    let passwd = getpwuid(uid);

    let mut stdout = stdout();

    match passwd {
        Some(passwd) => writeln!(&mut stdout, "{}", passwd.name)?,
        None => eprintln!("cannot find name for user ID {}", uid.as_raw()),
    }

    Ok(())
}
