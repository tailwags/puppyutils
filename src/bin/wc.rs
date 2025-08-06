use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write, stdin, stdout},
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

fn count_lines<R: BufRead>(mut reader: R) -> Result<(usize, usize, usize, usize)> {
    let mut lines = 0usize;
    let mut words = 0usize;
    let mut chars = 0usize;
    let mut bytes = 0usize;
    let mut line = String::new();

    loop {
        let read_bytes = reader.read_line(&mut line)?;

        if read_bytes == 0 {
            break;
        }

        bytes += read_bytes;
        lines += 1;
        words += line.split_whitespace().count();
        chars += line.chars().count();
        line.clear();
    }

    Ok((lines, words, chars, bytes))
}

enum StdioOrValue {
    Value(String),
    Stdio,
}

impl<T: AsRef<str>> From<T> for StdioOrValue {
    fn from(value: T) -> Self {
        Self::Value(value.as_ref().into())
    }
}

pub fn main() -> Result {
    let mut stdout = stdout();
    let mut files: Vec<StdioOrValue> = Vec::new();

    let mut flags = Flags::empty();

    cli! {
        "wc", stdout, #error
        Short('l') | Long("lines") => flags |= Flags::LINES
        Short('w') | Long("words") => flags |= Flags::WORDS
        Short('m') | Long("chars") => flags |= Flags::CHARS
        Short('c') | Long("bytes") => flags |= Flags::BYTES
        Value(value) => {
            files.push(value.into());
        }
        Stdio => files.push(StdioOrValue::Stdio)
    };

    if flags.is_empty() {
        flags = Flags::LINES | Flags::WORDS | Flags::BYTES;
    }

    let mut stdout = BufWriter::new(stdout);

    // If no files, read from stdin
    if files.is_empty() {
        files.push(StdioOrValue::Stdio);
    }

    for path in files {
        let (lines, words, chars, bytes) = match path {
            StdioOrValue::Value(ref value) => count_lines(BufReader::new(File::open(value)?))?,
            StdioOrValue::Stdio => count_lines(BufReader::new(stdin()))?,
        };

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

        if let StdioOrValue::Value(path) = path {
            stdout.write_all(b" ")?;
            stdout.write_all(path.as_bytes())?;
        }

        stdout.write_all(b"\n")?;
    }

    Ok(())
}
