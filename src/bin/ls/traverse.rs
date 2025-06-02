use bitflags::Flags;
use puppyutils::Result;
use rustix::fs::Dir;

use super::settings::{LsConfig, LsFlags};
use std::io::Write;

pub(crate) struct Printer<'a, O> {
    stdout: &'a mut O,
    cfg: LsConfig,
}

impl<'a, O> Printer<'a, O> {
    pub(crate) fn new(cfg: LsConfig, stdout: &'a mut O) -> Self
    where
        O: Write,
    {
        Self { stdout, cfg }
    }
}

impl<O: Write> Printer<'_, O> {
    pub(crate) fn traverse(&mut self, base_dir: Dir, dir_name: &str) -> Result {
        if self.cfg.flags.contains(LsFlags::RECURSIVE) {
            self.stdout.write_all(dir_name.as_bytes())?;
            self.stdout.write_all(b":\n")?;
        }

        todo!()
    }
}
