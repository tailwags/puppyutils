use std::{io::stdout, process::exit};

use puppyutils::{Result, cli_with_args};
use rustix::process::EXIT_FAILURE;
use sap::Parser;

pub fn main() -> Result {
    let args = std::env::args_os();

    if args.len() == 2 {
        let mut stdout = stdout();
        let mut args_parser = Parser::from_arbitrary(args)?;

        cli_with_args!(args_parser, "false", stdout, #ignore);
    }

    exit(EXIT_FAILURE);
}
