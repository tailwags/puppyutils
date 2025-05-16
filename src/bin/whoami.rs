use std::io::{Write, stdout};

use acumen::getpwuid;
use coreutils::Result;
use rustix::process::geteuid;

fn main() -> Result {
    let uid = geteuid();
    let passwd = getpwuid(uid);

    let mut stdout = stdout();

    match passwd {
        Some(passwd) => writeln!(&mut stdout, "{}", passwd.name)?,
        None => eprintln!("cannot find name for user ID {}", uid.as_raw()),
    }

    Ok(())
}
