use std::{borrow::Cow, env::current_exe, os::unix::ffi::OsStrExt, path::PathBuf};

use puppyutils::{Exit, Result};

pub mod bin {
    pub mod cat;
    pub mod r#false;
    #[path = "ls/main.rs"]
    pub mod ls;
    pub mod mkdir;
    pub mod pwd;
    pub mod touch;
    pub mod r#true;
    pub mod tty;
    pub mod uname;
    pub mod wc;
    pub mod whoami;
    pub mod yes;
}

fn main() -> Result {
    let util = match std::env::args_os().next() {
        Some(name) => PathBuf::from(name),
        None => current_exe()?,
    };

    let util = util
        .file_stem()
        .ok_or::<Exit>("Failed to get util name".into())?;

    match util.as_bytes() {
        b"cat" => bin::cat::main(),
        b"false" => bin::r#false::main(),
        b"ls" => bin::ls::main(),
        b"mkdir" => bin::mkdir::main(),
        b"pwd" => bin::pwd::main(),
        b"touch" => bin::touch::main(),
        b"true" => bin::r#true::main(),
        b"tty" => bin::tty::main(),
        b"uname" => bin::uname::main(),
        b"wc" => bin::wc::main(),
        b"whoami" => bin::whoami::main(),
        b"yes" => bin::yes::main(),
        _ => Err(Exit::Custom(Cow::Owned(format!(
            "unknown utility \"{}\"",
            util.to_string_lossy()
        )))),
    }
}
