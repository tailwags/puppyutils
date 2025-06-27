use std::{
    borrow::Cow,
    io::{BufWriter, Write, stdout},
};

use puppyutils::{Result, cli};

pub fn main() -> Result {
    // No point in locking stdout since we only use it once in this program
    let mut stdout = stdout();

    let mut buffer = None;

    let arg_parser = cli! {
        "yes", stdout, #error
        Value(value) => {
            extend_buffer(&mut buffer, value);
        }
    };

    arg_parser
        .into_inner()
        .for_each(|arg| extend_buffer(&mut buffer, arg.into()));

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

#[inline]
fn extend_buffer(buffer: &mut Option<String>, arg: Cow<'_, str>) {
    if let Some(buffer) = buffer {
        buffer.push(' '); // Manually put the space
        buffer.push_str(arg.as_ref());
    } else {
        *buffer = Some(arg.into_owned())
    }
}
