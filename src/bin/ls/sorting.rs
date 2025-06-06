use super::options::SortOrder;
use super::traverse::LsDisplay;
use core::cmp::Ordering;

pub(crate) fn sorting_fn(order: SortOrder) -> fn(&LsDisplay, &LsDisplay) -> Ordering {
    match order {
        SortOrder::Name => sort_by_name,
        SortOrder::Size => sort_by_size,
        SortOrder::Extension => sort_by_extension,
        SortOrder::AccessTime => sort_by_access_time,
        SortOrder::Time => sort_by_time,
        SortOrder::Width => sort_by_width,
        SortOrder::Version => sort_by_version,
        SortOrder::None => sort_by_none,

        SortOrder::Directory => unimplemented!(),
    }
}

fn sort_by_none(_lhs: &LsDisplay, _rhs: &LsDisplay) -> Ordering {
    Ordering::Equal
}

fn sort_by_name(lhs: &LsDisplay, rhs: &LsDisplay) -> Ordering {
    lhs.file_name().cmp(rhs.file_name())
}

fn sort_by_size(lhs: &LsDisplay, rhs: &LsDisplay) -> Ordering {
    let lhs_stat = lhs
        .stat()
        .unwrap_or_else(|| unreachable!("`sort_by_size` requires a present `Stat`"));

    let rhs_stat = rhs
        .stat()
        .unwrap_or_else(|| unreachable!("`sort_by_size` requires a present `Stat`"));

    lhs_stat.stx_size.cmp(&rhs_stat.stx_size)
}

fn sort_by_extension(lhs: &LsDisplay, rhs: &LsDisplay) -> Ordering {
    let mut lhs_extension_start_ix = 0;

    // short circuit
    // if the names are the same
    if lhs.file_name() == rhs.file_name() {
        return Ordering::Equal;
    }

    for (ix, byte) in lhs.file_name().to_bytes().iter().enumerate() {
        if byte == &b'.' {
            lhs_extension_start_ix = ix;
        }
    }

    let mut rhs_extension_start_ix = 0;

    for (ix, byte) in rhs.file_name().to_bytes().iter().enumerate() {
        if byte == &b'.' {
            rhs_extension_start_ix = ix;
        }
    }

    lhs.file_name()[lhs_extension_start_ix..].cmp(&rhs.file_name()[rhs_extension_start_ix..])
}

fn sort_by_access_time(lhs: &LsDisplay, rhs: &LsDisplay) -> Ordering {
    let lhs_stat = lhs
        .stat()
        .unwrap_or_else(|| unreachable!("`sort_by_access_time` requires a present `Stat`"));

    let rhs_stat = rhs
        .stat()
        .unwrap_or_else(|| unreachable!("`sort_by_access_time` requires a present `Stat`"));

    lhs_stat.stx_atime.tv_sec.cmp(&rhs_stat.stx_atime.tv_sec)
}

fn sort_by_time(lhs: &LsDisplay, rhs: &LsDisplay) -> Ordering {
    let lhs_stat = lhs
        .stat()
        .unwrap_or_else(|| unreachable!("`sort_by_time` requires a present `Stat`"));

    let rhs_stat = rhs
        .stat()
        .unwrap_or_else(|| unreachable!("`sort_by_time` requires a present `Stat`"));

    lhs_stat.stx_mtime.tv_sec.cmp(&rhs_stat.stx_mtime.tv_sec)
}

fn sort_by_width(lhs: &LsDisplay, rhs: &LsDisplay) -> Ordering {
    // might be naive
    // but that is what impls of ls
    // seem to do
    lhs.file_name()
        .to_bytes()
        .len()
        .cmp(&rhs.file_name().to_bytes().len())
}

fn sort_by_version(lhs: &LsDisplay, rhs: &LsDisplay) -> Ordering {
    if lhs.file_name() == rhs.file_name() {
        return Ordering::Equal;
    }

    let mut lhs_iter = lhs.file_name().to_bytes().iter();
    let mut rhs_iter = rhs.file_name().to_bytes().iter();

    while let Some(left) = lhs_iter.next()
        && let Some(right) = rhs_iter.next()
    {
        // If not a digit, just compare by byte value
        if !left.is_ascii_digit() || !right.is_ascii_digit() {
            if left != right {
                return left.cmp(right);
            }

            continue;
        }

        // Trailing zeroes
        if *left == b'0' && *right == b'0' {
            let mut zero_count_lhs = 0;

            while let Some(0) = lhs_iter.next() {
                zero_count_lhs += 1;
            }

            let mut zero_count_rhs = 0;

            while let Some(0) = rhs_iter.next() {
                zero_count_rhs += 1;
            }

            if zero_count_lhs != zero_count_rhs {
                return zero_count_lhs.cmp(&zero_count_lhs);
            }

            // special case here
        } else {
            // handling of actual digits
        }
    }
    todo!()
}
