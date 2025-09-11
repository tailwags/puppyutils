use std::io::stdout;

use puppyutils::{Result, cli};
use xenia::{AtFlags, ClockId, Timestamps, clock_gettime, stdio::cwd, utimensat};

pub fn main() -> Result {
    let mut stdout = stdout();
    let mut files = Vec::new();

    cli! {
        "touch", stdout, #error
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
        utimensat(cwd(), file, &timestamps, AtFlags::empty())?
    }

    Ok(())
}
