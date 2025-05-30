use std::io::{Write, stdout};

use puppyutils::{Result, cli};
use rustix::process::getcwd;
fn main() -> Result {
    let mut stdout = stdout();
    cli!("pwd", stdout, #error);

    let p = getcwd(Vec::new())?;

    stdout.write_all(p.as_bytes())?;
    stdout.write_all(b"\n")?;

    Ok(())
}
