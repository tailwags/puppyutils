use coreutils::{Result, cli_with_args};
use sap::Parser;
use std::io::stdout;

fn main() -> Result {
    let args = std::env::args_os();

    if args.len() == 2 {
        let mut stdout = stdout();
        let mut args_parser = Parser::from_arbitrary(args)?;

        cli_with_args!(args_parser, "true", stdout, #ignore);
    }

    Ok(())
}
