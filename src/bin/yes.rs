use std::{
    borrow::Cow,
    io::{self, BufWriter, Write},
    os::unix::ffi::OsStringExt,
};

fn main() -> io::Result<()> {
    // Creates a handle to stdout and wraps it into an in memory buffer.
    // No point in locking stdout since we only use it once in this program
    let mut out = BufWriter::new(std::io::stdout());

    // We can easily avoid the overhead of utf-8 since this is unix anyway
    let mut args = std::env::args_os();
    args.next(); // Calling next once is actually more efficient than using skip, we need this to skip the program name itself

    // We prepare the output so it doesn't need to go through the formatting each time
    let output: Cow<'_, [u8]> = if let Some(msg) = args.next() {
        let mut first = msg.into_vec(); // The first argument is always ready and we have a preallocated vec, we can just reuse it

        while let Some(next) = args.next() {
            first.push(b' '); // Manually put the space
            first.append(&mut next.into_vec()); // Append will move the data efficiently from the other vector
        }

        first.push(b'\n');

        Cow::Owned(first)
    } else {
        Cow::Borrowed(b"y\n") // If there are no args we can just hardcode it and avoid allocation
    };

    // Write everything to stdout, BufWriter will handle the buffering
    loop {
        out.write_all(&output)?;
    }
}
