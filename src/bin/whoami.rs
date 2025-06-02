use std::io::{Write, stderr, stdout};

use acumen::getpwuid;
use puppyutils::{Result, cli};
use rustix::process::geteuid;

const CANNOT_FIND_UID: &[u8] = b"cannot find name for user ID: ";

pub fn main() -> Result {
    let mut stdout = stdout();

    cli!("whoami", stdout, #error);

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
