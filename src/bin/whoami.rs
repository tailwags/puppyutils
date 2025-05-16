use std::io::{Write, stdout};

use acumen::getpwuid;
use coreutils::{Exit, Result};
use lexopt::prelude::*;
use rustix::process::geteuid;

fn main() -> Result {
    let mut arg_parser = lexopt::Parser::from_env();

    while let Some(arg) = arg_parser.next()? {
        match arg {
            Long("version") | Short('v') => {
                println!("puppyutils 0.0.1"); // TODO: properly generate this string
                return Ok(());
            }
            Long("help") | Short('h') => {
                println!(
                    "Usage: whoami\nPrint the user name associated with the current effective user ID."
                );
                return Ok(());
            }
            _ => return Err(Exit::ArgError(arg.unexpected())),
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
