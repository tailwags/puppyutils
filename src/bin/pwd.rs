use std::{
    io::{Write, stdout},
    os::unix::ffi::OsStringExt,
    path::PathBuf,
};

use puppyutils::{Result, cli};
use xenia::{getcwd, stat};

pub fn main() -> Result {
    let mut stdout = stdout();

    let mut logical = false;

    cli! {
        "pwd", stdout, #error
        Short('L') | Long("logical")  => logical = true
        Short('P') | Long("physical") => logical = false
    };

    let path = if logical
        && let Some(path) = std::env::var_os("PWD").map(PathBuf::from)
        && path.has_root()
        && let (Ok(pwd), Ok(dot)) = (stat(&path), stat(c"."))
        && pwd.st_dev == dot.st_dev
        && pwd.st_ino == dot.st_ino
    {
        path.into_os_string().into_vec()
    } else {
        getcwd(Vec::new())?.into_bytes()
    };

    stdout.write_all(&path)?;
    stdout.write_all(b"\n")?;

    Ok(())
}
