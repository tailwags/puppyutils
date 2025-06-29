use std::{
    borrow::Cow,
    io::{BufWriter, Write, stdout},
};

use puppyutils::{Result, cli};

pub fn main() -> Result {
    let mut stdout = stdout();

    let mut buffer: Option<String> = None;

    cli! {
        "yes", stdout, #error
        Value(value) => {
             if let Some(ref mut buffer) = buffer {
                buffer.push(' '); // Manually put the space
                buffer.push_str(value.as_ref());
            } else {
                buffer = Some(value.into_owned())
            }
        }
    };

    let output = if let Some(mut buffer) = buffer {
        buffer.push('\n');
        Cow::Owned(buffer)
    } else {
        Cow::Borrowed("y\n")
    };

    // Write everything to stdout, BufWriter will handle the buffering
    let mut stdout = BufWriter::new(stdout);

    loop {
        stdout.write_all(output.as_bytes())?;
    }
}
