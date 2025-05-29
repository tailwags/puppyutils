use std::io::{Write, stdout};

use coreutils::{Result, cli};
use rustix::system::uname;

bitflags::bitflags! {
    #[rustfmt::skip]
    pub struct Info: u8 {
        const KERNEL_NAME =         1 << 0;
        const NODENAME =            1 << 1;
        const KERNEL_RELEASE =      1 << 2;
        const KERNEL_VERSION =      1 << 3;
        const MACHINE =             1 << 4;
        const PROCESSOR =           1 << 5;
        const HARDWARE_PLATFORM =   1 << 6;
        const OPERATING_SYSTEM =    1 << 7;
    }
}

fn main() -> Result {
    let mut info_mask = Info::empty();
    let mut stdout = stdout();

    cli! {
        "uname", stdout, #error
        Short('a') | Long("all") => {
            info_mask = Info::all();
        }
        Short('s') | Long("kernel-name") => info_mask |= Info::KERNEL_NAME
        Short('n') | Long("nodename") => info_mask |= Info::NODENAME
        Short('r') | Long("kernel-release") => info_mask |= Info::KERNEL_RELEASE
        Short('v') | Long("kernel-version") => info_mask |= Info::KERNEL_VERSION
        Short('m') | Long("machine") => info_mask |= Info::MACHINE
        Short('p') | Long("processor") => info_mask |= Info::PROCESSOR
        Short('i') | Long("hardware-platform") => info_mask |= Info::HARDWARE_PLATFORM
        Short('o') | Long("operating-system") => info_mask |= Info::OPERATING_SYSTEM
    };

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
