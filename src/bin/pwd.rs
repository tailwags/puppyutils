use std::io::stdout;

use coreutils::{Result, cli};
fn main() -> Result {
    let mut stdout = stdout();
    cli!("pwd", stdout, #error);

    Ok(())
}
