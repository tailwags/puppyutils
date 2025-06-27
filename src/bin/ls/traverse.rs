// temporary
#![allow(dead_code)]

use super::options::Formatting;
use super::settings::{LsConfig, LsFlags};
use super::sorting;
use acumen::{Passwd, getpwuid};
use core::cmp;
use puppyutils::Result;

use rustix::ffi;
use rustix::fs::{AtFlags, Dir, Mode, OFlags, Statx, StatxFlags, Uid, open, statx};

use std::ffi::CStr;
use std::ffi::CString;
use std::io::Write;
use std::os::fd::OwnedFd;

/* File type masks */
const SOCKET: u16 = 0o0140000;
const SYMBOLIC_LINK: u16 = 0o0120000;
const REGULAR_FILE: u16 = 0o0100000;
const BLOCK_DEVICE: u16 = 0o0060000;
const DIRECTORY: u16 = 0o0040000;
const CHAR_DEVICE: u16 = 0o0020000;
const FIFO_NAMED_PIPE: u16 = 0o0010000;

/// The size required by a string of text
/// representing a human readable file size
///
/// like `5.1K` or `51M`
const HUMAN_READABLE_SIZE_LENGTH: usize = 4;

/// Stores state that is related
/// to printing out the `ls` output
///
/// also stores a mutable reference to
/// a `Write` object.
pub(crate) struct Printer<'a, O> {
    stdout: &'a mut O,
    cfg: LsConfig,
    fd: OwnedFd,
    base_dir: String,

    // longest file size
    longest_size: u64,

    // longest owner name (+ author name)
    longest_owner: usize,

    // longest symlink amount
    longest_symlink: usize,

    // longest group name,
    longest_group_name: usize,
}

impl<'a, O> Printer<'a, O> {
    /// Creates a new `Printer`
    pub(crate) fn new(cfg: LsConfig, stdout: &'a mut O) -> Result<Self>
    where
        O: Write,
    {
        let base_dir = cfg.directory();

        let fd = open(base_dir, OFlags::DIRECTORY | OFlags::RDONLY, Mode::RUSR)?;

        let res = Self {
            base_dir: base_dir.to_owned(),
            stdout,
            cfg,
            fd,
            longest_size: 0,
            longest_owner: 0,
            longest_symlink: 0,
            longest_group_name: 0,
        };

        Ok(res)
    }

    /// Checks if we shouldn't print the group column
    /// in long format
    fn no_groups(&self) -> bool {
        self.cfg.flags.contains(LsFlags::NO_GROUPS_LISTED)
    }

    /// Checks if we should print the author
    ///
    /// On Unix systems, this is going to just be the
    /// owner of the file
    fn show_file_author(&self) -> bool {
        self.cfg.flags.contains(LsFlags::PRINT_AUTHOR)
    }

    /// Checks if we shouldn't print the user column
    ///
    /// This is used by the `-g` parameter
    fn no_users(&self) -> bool {
        debug_assert!(
            matches!(self.cfg.format, Formatting::Long),
            "the `-g` option was used, however the formatting was not set to `Long`"
        );
        self.cfg.flags.contains(LsFlags::NO_OWNER_LISTED)
    }

    /// Checks if sizes should be printed in a human-readable form.
    ///
    /// like `3.4M`
    fn human_readable(&self) -> bool {
        self.cfg.flags.contains(LsFlags::HUMAN_READABLE_SIZES)
    }

    /// Checks if sizes should be calculated using SI units
    ///
    /// (powers of 10 instead of powers of 2)
    fn si_units(&self) -> bool {
        self.cfg.flags.contains(LsFlags::SI_SIZES)
    }

    // Do we show all files with `.` at the start
    fn show_dot_files(&self) -> bool {
        self.cfg.flags.contains(LsFlags::NOT_IGNORE_DOTS)
    }

    /// Do we show the current and parent directory entries
    ///
    /// (`.` and `..`)
    fn show_parent_and_current_directory(&self) -> bool {
        self.cfg.flags.contains(LsFlags::IGNORE_DOTS_EXCEPT_DIRS)
    }

    /// Is the format of formatting set to `Formatting::Long`
    ///
    /// for example by the `-l` option.
    fn is_long_format(&self) -> bool {
        matches!(self.cfg.format, Formatting::Long)
    }
}

impl<O: Write> Printer<'_, O> {
    /// Traverses the directory
    /// to form `LsDisplay` entries
    /// which will be passed forward
    /// for sorting and printing.
    pub(crate) fn traverse(&mut self) -> Result {
        let dir = Dir::read_from(&self.fd)?;
        let mut displays = Vec::with_capacity(16);
        self.construct(&mut displays, dir, self.needs_stat())?;

        // Somehow sort the vec of displays yeah
        displays.sort_by(sorting::sorting_fn(self.cfg.order));

        // find the longest values
        for display in &mut displays {
            // we need the longest:
            // size
            // symlink amount
            // user name
            // group name

            if let Some(ref stat) = display.stat {
                if self.human_readable() {
                    self.longest_size = HUMAN_READABLE_SIZE_LENGTH as u64
                } else {
                    self.longest_size =
                        cmp::max(self.longest_size, number_length_u64(stat.stx_size) as u64)
                }

                // maybe check if you actually need the user/group name
                if let Some(passwd) = getpwuid(Uid::from_raw(stat.stx_uid)) {
                    self.longest_owner = cmp::max(self.longest_owner, passwd.name.len());

                    display.passwd = Some(passwd) // unwrapping here is weird but we need the name length
                } else {
                    unreachable!("unable to obtain passwd for this uid") // redundant however nice for debugging
                }

                // group name
                self.longest_group_name = 7;

                // symlink amount
                self.longest_symlink = cmp::max(
                    self.longest_symlink,
                    number_length_u64(stat.stx_nlink as u64) as usize,
                );
            }
        }

        // find the longest values
        for display in &mut displays {
            // we need the longest:
            // size
            // symlink amount
            // user name
            // group name

            if let Some(ref stat) = display.stat {
                if self.human_readable() {
                    self.longest_size = HUMAN_READABLE_SIZE_LENGTH as u64
                } else {
                    self.longest_size =
                        cmp::max(self.longest_size, number_length_u64(stat.stx_size) as u64)
                }

                // maybe check if you actually need the user/group name
                if let Some(passwd) = getpwuid(Uid::from_raw(stat.stx_uid)) {
                    self.longest_owner = cmp::max(self.longest_owner, passwd.name.len());

                    display.passwd = Some(passwd) // unwrapping here is weird but we need the name length
                } else {
                    unreachable!("unable to obtain passwd for this uid") // redundant however nice for debugging
                }

                // group name
                self.longest_group_name = 7;

                // symlink amount
                self.longest_symlink = cmp::max(
                    self.longest_symlink,
                    number_length_u64(stat.stx_nlink.into()) as usize,
                );
            }
        }

        for display in displays {
            self.print_entry(display)?
        }

        Ok(())
    }

    fn needs_stat(&self) -> bool {
        true
    }

    /// Prints a singular entry
    ///
    /// Format specified by commandline arguments
    /// or the default
    fn print_entry(&mut self, display: LsDisplay) -> Result {
        if self.cfg.flags.contains(LsFlags::RECURSIVE) {
            self.stdout.write_all(display.file_name.to_bytes())?;
            self.stdout.write_all(b":\n")?;
        }

        match self.cfg.format {
            Formatting::Long => self.print_entry_long(display),
            _ => todo!(),
        }
    }

    /// Printing for `Formatting::Long`
    fn print_entry_long(&mut self, display: LsDisplay) -> Result {
        let stat = match display.stat {
            None => unreachable!("long format should have stat"),
            Some(ref stat) => stat,
        };

        // print file type
        print_filetype(self.stdout, stat)?;

        // print permissions
        let perms = Permissions::new(stat);
        perms.print_permissions(self.stdout)?;

        // print link amount
        let longest = self.longest_symlink;
        write!(self.stdout, "{:>longest$} ", stat.stx_nlink)?;

        if !self.no_users() {
            print_owner(self.stdout, stat.stx_uid, self.longest_owner)?;
        }

        if !self.no_groups() {
            print_group(self.stdout, stat.stx_gid, self.longest_group_name)?;
        }

        if self.show_file_author() {
            // on Unix, the file author and owner
            // are the same
            print_owner(self.stdout, stat.stx_uid, self.longest_owner)?;
        }

        print_size(
            self.stdout,
            self.human_readable(),
            self.si_units(),
            stat.stx_size,
            self.cfg.size_unit.map(Into::into).unwrap_or(1),
            self.longest_size as usize, // might be bad,
        )?;

        // change later to detect time
        let date = Date::new(stat.stx_ctime.tv_sec);
        print_date(self.stdout, date)?;

        self.stdout.write_all(display.file_name.to_bytes())?;
        self.stdout.write_all(b"\n")?;

        Ok(())
    }

    /// Constructs `LsDisplay` structs for printing
    fn construct(&mut self, displays: &mut Vec<LsDisplay>, dir: Dir, needs_stat: bool) -> Result {
        for entry in dir {
            let entry = entry?;
            let file_name = entry.file_name();

            if self.skip(file_name) {
                continue;
            }

            let mut display = LsDisplay {
                file_name: file_name.to_owned(),
                stat: None,
                group_name: None,
                passwd: None,
            };

            if needs_stat {
                let stat = statx(
                    &self.fd,
                    entry.file_name(),
                    AtFlags::empty(),
                    StatxFlags::MODE
                        | StatxFlags::GID
                        | StatxFlags::SIZE
                        | StatxFlags::TYPE
                        | StatxFlags::UID
                        | StatxFlags::BTIME
                        | StatxFlags::ATIME
                        | StatxFlags::MTIME
                        | StatxFlags::CTIME
                        | StatxFlags::BASIC_STATS
                        | StatxFlags::NLINK,
                )?;

                if self.is_long_format() {
                    let new_size = if self.human_readable() {
                        HUMAN_READABLE_SIZE_LENGTH
                    } else {
                        number_length_u64(stat.stx_size) as usize
                    };

                    if new_size as u64 > self.longest_size {
                        self.longest_size = new_size as u64
                    }
                }

                // if let Some(group) = acumen::group::GroupEntries::new()? {};

                display.stat = Some(stat)
            }

            displays.push(display)
        }

        Ok(())
    }

    /// Determines whether an entry should be skipped
    fn skip(&self, file_name: &CStr) -> bool {
        let bytes = file_name.to_bytes();

        (!self.show_dot_files() && bytes.first().copied().is_some_and(|byte| byte == b'.'))
            || (!self.show_parent_and_current_directory() && (bytes == b".." || bytes == b"."))
    }
}

/// An "entry" of a singular file (or directory)
/// that will be printed out.
#[non_exhaustive]
pub(crate) struct LsDisplay {
    stat: Option<Statx>,
    file_name: CString,
    group_name: Option<String>,
    passwd: Option<Passwd>,
}

impl LsDisplay {
    pub(crate) fn stat(&self) -> Option<&Statx> {
        self.stat.as_ref()
    }

    pub(crate) fn file_name(&self) -> &CStr {
        &self.file_name
    }
}

/// Opaque struct to handle Unix permissions
struct Permissions(u16);

impl Permissions {
    const OWNER_ALL: u16 = 0o0700;
    const OWNER_READ: u16 = 0o0400;
    const OWNER_WRITE: u16 = 0o0200;
    const OWNER_EXEC: u16 = 0o0100;

    const GROUP_ALL: u16 = 0o0070;
    const GROUP_READ: u16 = 0o0040;
    const GROUP_WRITE: u16 = 0o0020;
    const GROUP_EXEC: u16 = 0o0010;

    const OTHERS_ALL: u16 = 0o0007;
    const OTHERS_READ: u16 = 0o0004;
    const OTHERS_WRITE: u16 = 0o0002;
    const OTHERS_EXEC: u16 = 0o0001;

    fn new(stat: &Statx) -> Self {
        Self(stat.stx_mode & 0o7777)
    }

    fn owner(&self) -> [u8; 3] {
        let mut perms = [b'-'; 3];

        if self.0 == Self::OWNER_ALL {
            perms = [b'r', b'w', b'x'];
            return perms;
        }

        if self.0 >= Self::OWNER_READ {
            perms[0] = b'r';
        }

        if self.0 >= Self::OWNER_WRITE {
            perms[1] = b'w';
        }

        if self.0 >= Self::OWNER_EXEC {
            perms[2] = b'x';
        }

        perms
    }

    fn group(&self) -> [u8; 3] {
        let mut perms = [b'-'; 3];

        if self.0 == Self::GROUP_ALL {
            perms = [b'r', b'w', b'x'];
            return perms;
        }

        if self.0 >= Self::GROUP_READ {
            perms[0] = b'r';
        }

        if self.0 >= Self::GROUP_WRITE {
            perms[1] = b'w';
        }

        if self.0 >= Self::GROUP_EXEC {
            perms[2] = b'x';
        }

        perms
    }

    fn others(&self) -> [u8; 3] {
        let mut perms = [b'-'; 3];

        if self.0 == Self::OTHERS_ALL {
            perms = [b'r', b'w', b'x'];
            return perms;
        }

        if self.0 >= Self::OTHERS_READ {
            perms[0] = b'r';
        }

        if self.0 >= Self::OTHERS_WRITE {
            perms[1] = b'w';
        }

        if self.0 >= Self::OTHERS_EXEC {
            perms[2] = b'x';
        }

        perms
    }

    fn print_permissions<O>(&self, out: &mut O) -> Result
    where
        O: Write,
    {
        out.write_all(&self.owner())?;
        out.write_all(&self.group())?;
        out.write_all(&self.others())?;

        out.write_all(b" ").map_err(Into::into)
    }
}

/// Prints the character representing a file type
/// into `out`
fn print_filetype<O>(out: &mut O, stat: &Statx) -> Result
where
    O: Write,
{
    let file_char = match stat.stx_mode & 0o170000 {
        SOCKET => b's',
        SYMBOLIC_LINK => b'l',
        REGULAR_FILE => b'-',
        BLOCK_DEVICE => b'b',
        DIRECTORY => b'd',
        CHAR_DEVICE => b'c',
        FIFO_NAMED_PIPE => b'p',

        _ => unreachable!("there are no more file types present"),
    };

    out.write_all(&[file_char]).map_err(Into::into)
}

/// Prints the user from the `uid`
fn print_owner<O>(out: &mut O, uid: ffi::c_uint, _width: usize) -> Result
where
    O: Write,
{
    let Some(passwd) = getpwuid(Uid::from_raw(uid)) else {
        // somehow error out?
        todo!()
    };

    write!(out, "{} ", passwd.name)?;
    Ok(())
}

/// Prints the group from the `gid`
fn print_group<O>(_out: &mut O, _gid: u32, _width: usize) -> Result
where
    O: Write,
{
    // let Some(group) = getpwuid(Uid::from_raw(gid)) else {
    //     // error out
    //     todo!()
    // };
    // write!(out, "{} ", group.name)?;

    Ok(())
}

/// Prints the size of the entry
///
/// if `si_units` is `true` it uses `1000` instead of `1024`
/// for division
///
/// if `human_readable` is `true` it writes sizes in a
/// human readable format (like `4.2M`)
fn print_size<O>(
    out: &mut O,
    human_readable: bool,
    si_units: bool,
    mut size: u64,
    scale: u64,
    width: usize,
) -> Result
where
    O: Write + ?Sized,
{
    if human_readable {
        let mut buf: [u8; HUMAN_READABLE_SIZE_LENGTH] = [0_u8; HUMAN_READABLE_SIZE_LENGTH];
        humanize_number(&mut (&mut buf as &mut [u8]), size, si_units);
    } else if scale == 1 {
        write!(out, "{size:>width$} ")?;
    } else {
        while size >= scale {
            size /= scale;
        }

        write!(out, "{size:>width$} ")?;
    }
    Ok(())
}

fn print_date<O>(out: &mut O, date: Date) -> Result
where
    O: Write + ?Sized,
{
    let month = date.month_as_str();
    let day = date.days;
    write!(out, "{month} {day:>2} ")?;

    if date.hours < 10 {
        write!(out, "0")?;
    }
    write!(out, "{}:", date.hours)?;

    if date.minutes < 10 {
        write!(out, "0")?;
    }
    write!(out, "{} ", date.minutes).map_err(Into::into)
}

fn humanize_number<O>(buf: &mut O, num: u64, si: bool)
where
    O: Write + ?Sized,
{
    const UNITS: [&str; 8] = ["", "K", "M", "G", "T", "P", "E", "Z"];

    let divisor: f64 = [1024.0, 1000.0][si as usize];
    let maximum_scale = 7;

    let mut scale = 0;
    let mut floating_num = num as f64;

    while (floating_num >= divisor) && scale < maximum_scale {
        floating_num /= divisor;
        scale += 1;
    }

    write!(&mut *buf, "{:.1}{}", floating_num, UNITS[scale]).expect("infallible");
}

pub(crate) struct Date {
    year: i64,
    month: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    weekday: i64,
}

impl Date {
    pub(crate) fn new(seconds: i64) -> Self {
        const MONTH_DAYS: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut minutes = seconds / 60;
        let seconds = seconds - minutes * 60;

        let mut hours = minutes / 60;
        minutes -= hours * 60;

        let mut days = hours / 24;
        hours -= days * 24;

        let mut year = 1970; // unix starts from 1970
        let mut week_day: i64 = 4; // on a thursday

        let mut month = 0; // this will be overwritten anyway

        loop {
            let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
            let days_in_a_year = 365 + i64::from(leap_year);

            if days >= days_in_a_year {
                week_day += 1 + i64::from(leap_year);
                days -= days_in_a_year;

                if week_day >= 7 {
                    week_day -= 7;
                    year += 1;
                }
            } else {
                week_day += days;
                week_day %= days;

                for (month_index, month_day) in MONTH_DAYS.iter().enumerate() {
                    month = month_index as i64;
                    let cmp = *month_day + i64::from(month_index == 1 && leap_year);

                    if days >= cmp {
                        days -= cmp;
                    } else {
                        break;
                    }
                }

                break;
            }
        }

        month += 1;

        Self {
            year,
            month,
            days,
            hours,
            minutes,
            seconds,
            weekday: week_day,
        }
    }
    pub(crate) fn month_as_str(&self) -> &'static str {
        const MONTH_NAMES: [&str; 13] = [
            "", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];

        debug_assert!(self.month != 0, "month was set to 0, which is invalid");

        MONTH_NAMES[self.month as usize]
    }

    pub(crate) fn hours_minutes_as_str<'a, 'b>(&'a self, buf: &'b mut [u8]) -> Option<&'b str>
    where
        'b: 'a,
    {
        const END_OF_BUFFER: usize = 5;

        if buf.len() < 5 {
            return None;
        }

        write!(&mut *buf, "{}:{}", self.hours, self.minutes)
            .expect("infallible, this is a memory buffer");

        unsafe { Some(core::str::from_utf8_unchecked(&buf[..END_OF_BUFFER])) }
    }

    /// Turns the `days` in the `Date` struct
    /// into a padded value that is written
    /// into `buf`
    ///
    /// Example:
    ///
    /// `9`  -> `" 9"`
    /// `20` -> `"10"`
    ///
    /// if `buf` doesn't have a length of atleast 2
    /// the day written may be malformed.
    pub(crate) fn day_with_padding(&self, mut buf: &mut [u8]) -> Result {
        if self.days < 10 {
            write!(buf, " ")?;
        };

        write!(buf, "{}", self.days).map_err(Into::into)
    }
}

#[inline]
fn number_length_u64(n: u64) -> u32 {
    match n {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1000..=9999 => 4,
        10000..=99999 => 5,
        100000..=999999 => 6,
        1000000..=9999999 => 7,
        100000000..=999999999 => 8,
        1000000000..=9999999999 => 9,
        10000000000..=99999999999 => 10,
        100000000000..=999999999999 => 11,
        1000000000000..=9999999999999 => 12,
        10000000000000..=99999999999999 => 13,
        100000000000000..=999999999999999 => 14,
        1000000000000000..=9999999999999999 => 15,
        10000000000000000..=99999999999999999 => 16,
        100000000000000000..=999999999999999999 => 17,
        1000000000000000000..=9999999999999999999 => 18,
        10000000000000000000..=18446744073709551615 => 19,
        _ => unsafe { std::hint::unreachable_unchecked() },
    }
}
