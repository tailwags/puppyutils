use std::{
    borrow::Cow,
    io::{BufWriter, Write, stdout},
    os::unix::ffi::OsStringExt,
};

use coreutils::{Result, cli};

fn main() -> Result {
    // No point in locking stdout since we only use it once in this program
    let mut stdout = stdout();

    let mut buffer: Option<Vec<u8>> = None;

    let arg_parser = cli! {
        "yes", stdout, while,
        Value(value) => {
            extend_buffer(&mut buffer, value.as_bytes().to_vec());
        }
        arg => return Err(arg.into_error(None).into())
    };

    arg_parser
        .into_inner()
        .for_each(|arg| extend_buffer(&mut buffer, arg.into_vec()));

    let output: Cow<'_, [u8]> = if let Some(mut buffer) = buffer {
        buffer.push(b'\n');
        Cow::Owned(buffer)
    } else {
        Cow::Borrowed(b"y\n")
    };

    // Write everything to stdout, BufWriter will handle the buffering
    let mut stdout = BufWriter::new(stdout);

    loop {
        stdout.write_all(&output)?;
    }
}

#[inline]
fn extend_buffer(buffer: &mut Option<Vec<u8>>, mut arg: Vec<u8>) {
    if let Some(buffer) = buffer {
        buffer.push(b' '); // Manually put the space
        buffer.append(&mut arg);
    } else {
        *buffer = Some(arg)
    }
}
