use std::io::{Write, stdout};

use coreutils::Result;
use rustix::system::uname;

fn main() -> Result {
    let uname = uname();

    let mut stdout = stdout();

    stdout.write_all(uname.sysname().to_bytes())?;
    stdout.write_all(b" ")?;

    stdout.write_all(uname.nodename().to_bytes())?;
    stdout.write_all(b" ")?;

    stdout.write_all(uname.release().to_bytes())?;
    stdout.write_all(b" ")?;

    stdout.write_all(uname.version().to_bytes())?;
    stdout.write_all(b" ")?;

    stdout.write_all(uname.machine().to_bytes())?;

    stdout.write_all(b"\n")?;

    stdout.flush()?;

    Ok(())
}
