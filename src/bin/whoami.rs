use std::io::{Write, stderr, stdout};

use puppyutils::{Result, cli};
use xenia::geteuid;
use xenia_utils::passwd::Passwd;

const CANNOT_FIND_UID: &[u8] = b"cannot find name for user ID: ";

pub fn main() -> Result {
    let mut stdout = stdout();

    cli!("whoami", stdout, #error);

    let uid = geteuid();

    let mut parser = Passwd::entries()?;

    while let Ok(Some(entry)) = parser.next_entry() {
        if entry.uid == uid {
            stdout.write_all(entry.name.as_bytes())?;
            stdout.write_all(b"\n")?;

            stdout.flush()?;

            return Ok(());
        }
    }

    let mut stderr = stderr();

    stderr.write_all(CANNOT_FIND_UID)?;
    stderr.write_all(itoa::Buffer::new().format(uid.as_raw()).as_bytes())?;
    stderr.write_all(b"\n")?;

    stderr.flush()?;

    Ok(())
}
