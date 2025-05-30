#![feature(exitcode_exit_method)]

use std::{io::stdout, process::ExitCode};

use puppyutils::{Result, cli_with_args};
use sap::Parser;

fn main() -> Result {
    let args = std::env::args_os();

    if args.len() == 2 {
        let mut stdout = stdout();
        let mut args_parser = Parser::from_arbitrary(args)?;

        cli_with_args!(args_parser, "false", stdout, #ignore);
    }

    ExitCode::FAILURE.exit_process()
}
