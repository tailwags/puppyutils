use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Write, stdin, stdout},
};

use puppyutils::{Result, cli};

pub fn main() -> Result {
    let mut stdout = stdout();
    let mut files = Vec::new();

    cli! {
        "cat", stdout, #error
        Value(value) => {
            files.push(value.into_owned());
        }
    };

    let mut stdout = BufWriter::new(stdout);

    // FIXME: so much repetition, can we do better?
    if files.is_empty() {
        let mut stdin = BufReader::new(stdin());
        io::copy(&mut stdin, &mut stdout)?;
    } else {
        for file_path in files {
            // "-" means stdin
            if file_path == "-" {
                // TODO: maybe the handling for - should be at the arg parser stage?
                let mut stdin = BufReader::new(stdin());
                io::copy(&mut stdin, &mut stdout)?;
            } else {
                let mut reader = BufReader::new(File::open(file_path)?); // Is it fine to error here? or should we keep going
                io::copy(&mut reader, &mut stdout)?;
            }
        }
    }

    stdout.flush()?;

    Ok(())
}
