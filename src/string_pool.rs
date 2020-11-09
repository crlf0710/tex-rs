#[distributed_slice]
pub(crate) static STRPL_RAWSTRS: [&'static str] = [..];

static STRPOOL_ITEMS_FROM_256: Lazy<Vec<&'static str>> = Lazy::new(prepare_compiletime_string_pool);

fn prepare_compiletime_string_pool() -> Vec<&'static str> {
    let mut existing = BTreeSet::new();
    let mut result = vec![];
    for str in STRPL_RAWSTRS.iter().cloned() {
        if str.len() == 1 {
            continue;
        }
        if existing.contains(&str) {
            continue;
        }
        result.push(str);
        existing.insert(str);
    }
    result
}

pub(crate) fn string_pool_index(val: &'static str) -> usize {
    if val.len() == 1 {
        return val.as_bytes()[0] as usize;
    }
    for (idx, str) in (256usize..).zip(STRPOOL_ITEMS_FROM_256.iter().cloned()) {
        if val == str {
            trace_expr!("string_pool[{}] = \"{}\"", idx, str);
            return idx;
        }
    }
    unreachable!();
}

static POOL_FILE: Lazy<Vec<u8>> = Lazy::new(generate_initial_memory_pool_file);

static CHECKSUM: Lazy<usize> = Lazy::new(generate_checksum);

pub(crate) fn generate_initial_memory_pool_file() -> Vec<u8> {
    let mut cursor = io::Cursor::new(vec![]);
    for str in STRPOOL_ITEMS_FROM_256.iter().cloned() {
        assert!(str.len() < 256);
        write!(cursor, "{:02}{}\n", str.len(), str).unwrap();
    }
    write!(cursor, "*{:09}", *CHECKSUM).unwrap();
    use std::io::Write;

    cursor.into_inner()
}

pub(crate) fn generate_checksum() -> usize {
    123456789
}

pub(crate) fn pool_file() -> io::Cursor<&'static [u8]> {
    io::Cursor::new(&*POOL_FILE)
}

use linkme::distributed_slice;
use once_cell::sync::Lazy;
use std::collections::BTreeSet;
use std::io;
