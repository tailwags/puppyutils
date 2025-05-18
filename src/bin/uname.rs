use std::io::{Write, stdout};

use coreutils::Result;
use rustix::system::uname;
use sap::{
    Argument::{Long, Short},
    Parser,
};

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct Info: u8 {
        const KERNEL_NAME = 0b00000001;
        const NODENAME = 0b00000010;
        const KERNEL_RELEASE = 0b00000100;
        const KERNEL_VERSION = 0b00001000;
        const MACHINE = 0b00010000;
        const PROCESSOR = 0b00100000;
        const HARDWARE_PLATFORM = 0b01000000;
        const OPERATING_SYSTEM = 0b10000000;
    }
}

const HELP_TEXT: &str = "Usage: uname [OPTION]...
Print system information.

  -a, --all                print all information
  -s, --kernel-name        print the kernel name (default)
  -n, --nodename           print the network node hostname
  -r, --kernel-release     print the kernel release
  -v, --kernel-version     print the kernel version
  -m, --machine            print the machine hardware name
  -p, --processor          print the processor type
  -i, --hardware-platform  print the hardware platform
  -o, --operating-system   print the operating system
      --help               display this help and exit
      --version            output version information and exit

With no OPTION, same as -s.
";

const VERSION: &str = coreutils::version_text!("uname", "0.0.1");

fn main() -> Result {
    let mut info_mask = Info::empty();
    let mut stdout = stdout();

    let mut arg_parser = Parser::from_env()?;

    while let Some(arg) = arg_parser.forward()? {
        match arg {
            Long("version") => {
                // stdout.write_all(b"puppyutils 0.0.1\n")?;
                stdout.write_all(VERSION.as_bytes())?;
                stdout.flush()?;
                return Ok(());
            }
            Long("help") => {
                stdout.write_all(HELP_TEXT.as_bytes())?;
                stdout.flush()?;
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
            Short('p') | Long("processor") => info_mask |= Info::PROCESSOR,
            Short('i') | Long("hardware-platform") => info_mask |= Info::HARDWARE_PLATFORM,
            Short('o') | Long("operating-system") => info_mask |= Info::OPERATING_SYSTEM,

            argument => return Err(argument.into_error(None).into()),
        }
    }

    if info_mask.is_empty() {
        info_mask = Info::KERNEL_NAME;
    }

    let uname = uname();
    let mut first = true;

    // Helper function to write field with space handling
    let mut write_field = |data: &[u8]| -> Result<()> {
        if !first {
            stdout.write_all(b" ")?;
        }
        stdout.write_all(data)?;
        first = false;
        Ok(())
    };

    if info_mask.contains(Info::KERNEL_NAME) {
        write_field(uname.sysname().to_bytes())?;
    }

    if info_mask.contains(Info::NODENAME) {
        write_field(uname.nodename().to_bytes())?;
    }

    if info_mask.contains(Info::KERNEL_RELEASE) {
        write_field(uname.release().to_bytes())?;
    }

    if info_mask.contains(Info::KERNEL_VERSION) {
        write_field(uname.version().to_bytes())?;
    }

    if info_mask.contains(Info::MACHINE) {
        write_field(uname.machine().to_bytes())?;
    }

    #[allow(clippy::collapsible_if)]
    if info_mask.contains(Info::PROCESSOR) {
        if !info_mask.is_all() {
            // TODO: figure out if there is anything to do here
            write_field(b"unknown")?;
        }
    }

    #[allow(clippy::collapsible_if)] // FIXME: remove this once we figure out what to do below
    if info_mask.contains(Info::HARDWARE_PLATFORM) {
        if !info_mask.is_all() {
            // TODO: figure out if there is anything to do here
            write_field(b"unknown")?;
        }
    }

    #[allow(clippy::collapsible_if)]
    if info_mask.contains(Info::OPERATING_SYSTEM) {
        let os: &[u8] = match uname.sysname().to_bytes() {
            b"Linux" => b"GNU/Linux",
            _ => b"unknown",
        };

        write_field(os)?;
    }

    stdout.write_all(b"\n")?;
    stdout.flush()?;

    Ok(())
}
