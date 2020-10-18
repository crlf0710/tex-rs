#[distributed_slice]
pub(crate) static STRPL_STRS: [&'static str] = [..];

pub(crate) fn string_pool_index(val: &'static str) -> usize {
    for (idx, str) in STRPL_STRS.iter().cloned().enumerate() {
        if val == str {
            return idx;
        }
    }
    unreachable!();
}

use linkme::distributed_slice;
