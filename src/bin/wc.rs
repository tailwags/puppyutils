use std::{
    fs::File,
    io::{BufRead, BufReader, Write, stdout},
};

use puppyutils::{Result, cli};

bitflags::bitflags! {
    struct Flags: u8 {
        const LINES = 1 << 0;
        const WORDS = 1 << 1;
        const CHARS = 1 << 2;
        const BYTES = 1 << 3;
    }
}

pub fn main() -> Result {
    let mut stdout = stdout();
    let mut files = Vec::new();

    let mut flags = Flags::empty();

    cli! {
        "wc", stdout, #error
        Short('l') | Long("lines") => flags |= Flags::LINES
        Short('w') | Long("words") => flags |= Flags::WORDS
        Short('m') | Long("chars") => flags |= Flags::CHARS
        Short('c') | Long("bytes") => flags |= Flags::BYTES
        Value(value) => {
            files.push(value.to_owned());
        }
    };

    #[allow(unused)]
    if flags.is_empty() {
        flags = Flags::LINES | Flags::WORDS | Flags::BYTES;
    }

    for path in files {
        let file = File::open(&path)?;

        let mut bytes = file.metadata().map(|m| m.len()).unwrap_or(0) as usize;

        let mut lines = 0usize;
        let mut words = 0usize;
        let mut chars = 0usize;

        let mut reader = BufReader::new(file);

        let mut line = String::new();
        loop {
            let read_bytes = reader.read_line(&mut line)?;

            if read_bytes == 0 {
                break;
            }

            if bytes == 0 {
                bytes += read_bytes;
            }

            lines += 1;
            words += line.split_whitespace().count();
            chars += line.chars().count();

            line.clear();
        }

        let out = format!("  {lines}  {words} {chars} {bytes} {path}\n"); // FIXME: obv we dont wanna format but also this is temporary since we are ignoring the flags
        stdout.write_all(out.as_bytes())?;
    }

    Ok(())
}
