use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write, stdout},
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

    let mut stdout = BufWriter::new(stdout);

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

        let mut first = true;

        let mut write_num = |num: usize| -> Result<()> {
            if !first {
                stdout.write_all(b" ")?;
            }
            stdout.write_all(itoa::Buffer::new().format(num).as_bytes())?;
            first = false;
            Ok(())
        };

        if flags.contains(Flags::LINES) {
            write_num(lines)?;
        }

        if flags.contains(Flags::WORDS) {
            write_num(words)?;
        }

        if flags.contains(Flags::CHARS) {
            write_num(chars)?;
        }

        if flags.contains(Flags::BYTES) {
            write_num(bytes)?;
        }

        stdout.write_all(b" ")?;
        stdout.write_all(path.as_bytes())?;
        stdout.write_all(b"\n")?;
    }

    Ok(())
}

fn _count_padding(mut n: usize) -> usize {
    let mut count = 0;
    while n % 10 == 0 {
        n /= 10;
        count += 1;
    }
    count
}
