use std::{ffi::CStr, io::stdout};

use puppyutils::{Result, cli};
use xenia::{
    AtFlags, ClockId, Errno, Mode, OFlags, Timestamps, clock_gettime, open, stdio::cwd, utimensat,
};

pub fn main() -> Result {
    let mut stdout = stdout();
    let mut files = Vec::new();

    let mut no_create = false;

    cli! {
        "touch", stdout, #error
        Short('c') | Long("no-create") => no_create = true
        Value(value) => {
            files.push(value.into_owned());
        }
    };

    let now = clock_gettime(ClockId::Realtime)?;

    let timestamps = Timestamps {
        last_access: now,
        last_modification: now,
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
