use std::{ffi::CStr, io::stdout};

use puppyutils::{Result, cli};
use xenia::{
    AtFlags, ClockId, Errno, Mode, OFlags, Timespec, Timestamps, clock_gettime, open, stdio::cwd,
    utimensat,
};

pub fn main() -> Result {
    let mut stdout = stdout();
    let mut files = Vec::new();

    let mut no_create = false;
    let mut access_time = true;
    let mut modification_time = true;

    cli! {
        "touch", stdout, #error
        Short('c') | Long("no-create") => no_create = true
        Short('a') => modification_time = false
        Short('m')=> access_time = false
        Value(value) => {
            files.push(value.into_owned());
        }
    };

    let now = if files.len() == 1 {
        Timespec::NOW
    } else {
        clock_gettime(ClockId::Realtime)?
    };

    #[rustfmt::skip]
    let timestamps = Timestamps {
        last_access: if access_time { now } else { Timespec::OMIT },
        last_modification: if modification_time { now } else { Timespec::OMIT },
    };

    for file in files {
        match utimensat(cwd(), &file, &timestamps, AtFlags::empty()) {
            Ok(_) => {}
            Err(errno) => {
                if no_create || errno != Errno::NOENT {
                    return Err(errno.into());
                }

                let fd = open(
                    file,
                    OFlags::WRONLY | OFlags::CREAT,
                    Mode::from_bits_retain(0o666),
                )?;

                utimensat::<_, &CStr>(fd, None, &timestamps, AtFlags::empty())?;
            }
        }
    }

    Ok(())
}
