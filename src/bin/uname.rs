use std::io::{Write, stdout};

use coreutils::{Exit, Result};
use lexopt::prelude::*;
use rustix::system::uname;

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct Info: u32 {
        const KERNEL_NAME = 0b00000001;
        const NODENAME = 0b00000010;
        const KERNEL_RELEASE = 0b00000100;
        const KERNEL_VERSION = 0b00001000;
        const MACHINE = 0b00010000;
    }
}

fn main() -> Result {
    let mut info_mask = Info::empty();

    let mut arg_parser = lexopt::Parser::from_env();

    while let Some(arg) = arg_parser.next()? {
        match arg {
            Long("version") => {
                println!("puppyutils 0.0.1"); // TODO: properly generate this string
                return Ok(());
            }
            Long("help") => {
                println!("Usage: uname"); // TODO: add all options
                return Ok(());
            }
            Short('a') | Long("all") => {
                info_mask = Info::all();
            }
            Short('s') | Long("kernel-name") => info_mask |= Info::KERNEL_NAME,
            Short('n') | Long("nodename") => info_mask |= Info::NODENAME,
            Short('r') | Long("kernel-release") => info_mask |= Info::KERNEL_RELEASE,
            Short('v') | Long("kernel-version") => info_mask |= Info::KERNEL_VERSION,
            Short('m') | Long("machine") => info_mask |= Info::MACHINE,
            _ => return Err(Exit::ArgError(arg.unexpected())),
        }
    }

    if info_mask.is_empty() {
        info_mask = Info::KERNEL_NAME
    }

    let uname = uname();

    let mut stdout = stdout();

    // FIXME: we should keep track of spaces to avoid traling ones
    if info_mask.contains(Info::KERNEL_NAME) {
        stdout.write_all(uname.sysname().to_bytes())?;
        stdout.write_all(b" ")?;
    }

    if info_mask.contains(Info::NODENAME) {
        stdout.write_all(uname.nodename().to_bytes())?;
        stdout.write_all(b" ")?;
    }

    if info_mask.contains(Info::KERNEL_RELEASE) {
        stdout.write_all(uname.release().to_bytes())?;
        stdout.write_all(b" ")?;
    }

    if info_mask.contains(Info::KERNEL_VERSION) {
        stdout.write_all(uname.version().to_bytes())?;
        stdout.write_all(b" ")?;
    }

    if info_mask.contains(Info::MACHINE) {
        stdout.write_all(uname.machine().to_bytes())?;
        stdout.write_all(b" ")?;
    }

    stdout.write_all(b"\n")?;

    stdout.flush()?;

    Ok(())
}
