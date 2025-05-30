use std::io::stdout;

use puppyutils::{Result, cli};
fn main() -> Result {
    let mut stdout = stdout();
    cli!("pwd", stdout, #error);

    Ok(())
}
