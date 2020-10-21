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

static POOL_FILE: Lazy<Vec<u8>> = Lazy::new(generate_memory_pool_file);

pub(crate) fn generate_memory_pool_file() -> Vec<u8> {
    // FIXME
    vec![]
}

pub(crate) fn pool_file() -> io::Cursor<&'static [u8]> {
    io::Cursor::new(&*POOL_FILE)
}

use linkme::distributed_slice;
use once_cell::sync::Lazy;
use std::io;
