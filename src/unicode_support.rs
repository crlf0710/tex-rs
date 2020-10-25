#![cfg(feature = "unicode_support")]

thread_local! {
    static GRAPHE_REGISTRY: GraphemeRegistry = GraphemeRegistry::new();
}

struct GraphemeRegistry {
    normalized_strings_after_single_scalar_value: RefCell<Vec<&'static str>>,
    normalized_string_lookup_map: RefCell<BTreeMap<&'static str, u32>>,
    next_value: Cell<u32>,
}

const GRAPHEME_REGISTRY_INITIAL_VALUE: u32 = core::char::MAX as u32 + 1;

impl GraphemeRegistry {
    fn new() -> Self {
        GraphemeRegistry {
            normalized_strings_after_single_scalar_value: RefCell::new(Vec::new()),
            normalized_string_lookup_map: RefCell::new(BTreeMap::new()),
            next_value: Cell::new(GRAPHEME_REGISTRY_INITIAL_VALUE),
        }
    }

    fn intern_normalized_multi_scalar_value(&self, s: String) -> u32 {
        let s_ref = &s[..];
        if let Some(v) = self.normalized_string_lookup_map.borrow_mut().get(s_ref) {
            return *v;
        }
        let cur_value = self.next_value.get();
        let s = Box::leak(s.into_boxed_str());
        self.normalized_strings_after_single_scalar_value
            .borrow_mut()
            .push(s);
        self.normalized_string_lookup_map
            .borrow_mut()
            .insert(s, cur_value);
        self.next_value.set(cur_value.checked_add(1).unwrap());
        cur_value
    }
}

enum GraphemeRegistryItem {
    SingleScalarValue(char),
    MultiScalarValue(&'static str),
    InvalidValue(u32),
}

enum GraphemeRegistryItemIter {
    RegItem(GraphemeRegistryItem),
    StrIter(std::str::Chars<'static>),
    None,
}

impl Iterator for GraphemeRegistryItemIter {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self {
            GraphemeRegistryItemIter::RegItem(item) => match item {
                GraphemeRegistryItem::SingleScalarValue(c) => {
                    let c = *c;
                    *self = GraphemeRegistryItemIter::None;
                    Some(c)
                }
                GraphemeRegistryItem::MultiScalarValue(s) => {
                    let mut chars = (*s).chars();
                    let next = chars.next();
                    *self = GraphemeRegistryItemIter::StrIter(chars);
                    next
                }
                GraphemeRegistryItem::InvalidValue(_) => {
                    let x = '\u{FFFD}';
                    *self = GraphemeRegistryItemIter::None;
                    Some(x)
                }
            },
            GraphemeRegistryItemIter::StrIter(iter) => iter.next(),
            GraphemeRegistryItemIter::None => None,
        }
    }
}

type generalized_char = crate::pascal::char;

pub(crate) fn generalized_char_from_str(s: &str) -> generalized_char {
    debug_assert!(!s.is_empty());
    let (pos, ch) = s.char_indices().rev().next().unwrap();
    if pos == 0 {
        // single scalar value, fast path
        return generalized_char::new(ch as u32);
    }
    let str = s.nfc().collect::<String>();
    let result = GRAPHE_REGISTRY.with(|reg| reg.intern_normalized_multi_scalar_value(str));
    generalized_char::new(result)
}

pub(crate) fn chars_from_generalized_char(val: generalized_char) -> impl Iterator<Item = char> {
    use core::convert::TryFrom;
    let val = val.0;
    let item = if val < GRAPHEME_REGISTRY_INITIAL_VALUE {
        match char::try_from(val) {
            Ok(v) => GraphemeRegistryItem::SingleScalarValue(v),
            Err(..) => GraphemeRegistryItem::InvalidValue(val),
        }
    } else {
        GRAPHE_REGISTRY.with(|reg| {
            let strings = reg.normalized_strings_after_single_scalar_value.borrow();
            match strings.get((val - GRAPHEME_REGISTRY_INITIAL_VALUE) as usize) {
                Some(v) => GraphemeRegistryItem::MultiScalarValue(v),
                None => GraphemeRegistryItem::InvalidValue(val),
            }
        })
    };
    GraphemeRegistryItemIter::RegItem(item)
}

/// fss-utf handling code adapted from rust standard library utf-8 handling.

#[derive(Copy, Clone)]
pub(crate) struct FssUtfEncodedIP32 {
    bytes: [u8; 6],
    len: u8,
}

const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;
const TAG_FIVE_B: u8 = 0b1111_1000;
const TAG_SIX_B: u8 = 0b1111_1100;
const MAX_ONE_B: i32 = 0x80;
const MAX_TWO_B: i32 = 0x800;
const MAX_THREE_B: i32 = 0x10000;
const MAX_FOUR_B: i32 = 0x20_0000;
const MAX_FIVE_B: i32 = 0x400_0000;

pub(crate) fn len_fss_utf(code: i32) -> usize {
    assert!(code >= 0);
    if code < MAX_ONE_B {
        1
    } else if code < MAX_TWO_B {
        2
    } else if code < MAX_THREE_B {
        3
    } else {
        4
    }
}

pub(crate) fn len_fss_utf_from_first_byte(v: u8) -> usize {
    if v < 128 {
        return 1;
    } else if v & TAG_SIX_B == TAG_SIX_B {
        return 6;
    } else if v & TAG_FIVE_B == TAG_FIVE_B {
        return 5;
    } else if v & TAG_FOUR_B == TAG_FOUR_B {
        return 4;
    } else if v & TAG_THREE_B == TAG_THREE_B {
        return 3;
    } else if v & TAG_TWO_B == TAG_TWO_B {
        return 2;
    } else {
        unreachable!()
    }
}

impl FssUtfEncodedIP32 {
    pub(crate) fn new(code: i32) -> Self {
        assert!(code >= 0);
        let mut bytes = [0; 6];
        let len = len_fss_utf(code);
        match (len, &mut bytes[..]) {
            (1, [a, ..]) => {
                *a = code as u8;
            }
            (2, [a, b, ..]) => {
                *a = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
                *b = (code & 0x3F) as u8 | TAG_CONT;
            }
            (3, [a, b, c, ..]) => {
                *a = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
                *b = (code >> 6 & 0x3F) as u8 | TAG_CONT;
                *c = (code & 0x3F) as u8 | TAG_CONT;
            }
            (4, [a, b, c, d, ..]) => {
                *a = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
                *b = (code >> 12 & 0x3F) as u8 | TAG_CONT;
                *c = (code >> 6 & 0x3F) as u8 | TAG_CONT;
                *d = (code & 0x3F) as u8 | TAG_CONT;
            }
            (5, [a, b, c, d, e, ..]) => {
                *a = (code >> 24 & 0x03) as u8 | TAG_FIVE_B;
                *b = (code >> 18 & 0x3F) as u8 | TAG_CONT;
                *c = (code >> 12 & 0x3F) as u8 | TAG_CONT;
                *d = (code >> 6 & 0x3F) as u8 | TAG_CONT;
                *e = (code & 0x3F) as u8 | TAG_CONT;
            }
            (6, [a, b, c, d, e, f, ..]) => {
                *a = (code >> 30 & 0x01) as u8 | TAG_SIX_B;
                *b = (code >> 24 & 0x3F) as u8 | TAG_CONT;
                *c = (code >> 18 & 0x3F) as u8 | TAG_CONT;
                *d = (code >> 12 & 0x3F) as u8 | TAG_CONT;
                *e = (code >> 6 & 0x3F) as u8 | TAG_CONT;
                *f = (code & 0x3F) as u8 | TAG_CONT;
            }
            _ => unreachable!(),
        };

        FssUtfEncodedIP32 {
            bytes,
            len: len as u8,
        }
    }
}

impl IntoIterator for FssUtfEncodedIP32 {
    type IntoIter = FssUtfEncodedIP32Iter;
    type Item = u8;
    fn into_iter(self) -> Self::IntoIter {
        FssUtfEncodedIP32Iter {
            value: self,
            cursor: 0,
        }
    }
}

pub(crate) struct FssUtfEncodedIP32Iter {
    value: FssUtfEncodedIP32,
    cursor: u8,
}

impl Iterator for FssUtfEncodedIP32Iter {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.cursor < self.value.len {
            let val = self.value.bytes[self.cursor as usize];
            self.cursor += 1;
            Some(val)
        } else {
            None
        }
    }
}

const CONT_MASK: u8 = 0b0011_1111;
pub(crate) struct GenericCharIter<'a> {
    slice: &'a [crate::section_0038::packed_ASCII_code],
}

impl<'a> GenericCharIter<'a> {
    pub(crate) fn new(slice: &'a [crate::section_0038::packed_ASCII_code]) -> Self {
        GenericCharIter { slice }
    }
}

#[inline]
fn fss_utf_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

#[inline]
fn fss_utf_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}

impl<'a> Iterator for GenericCharIter<'a> {
    type Item = generalized_char;

    fn next(&mut self) -> Option<generalized_char> {
        if self.slice.is_empty() {
            return None;
        }
        let byte1 = self.slice[0].0;
        if byte1 < 128 {
            self.slice = &self.slice[1..];
            return Some(generalized_char::new(byte1 as _));
        }
        let byte_cnt = len_fss_utf_from_first_byte(byte1);
        let mut val = fss_utf_first_byte(byte1, byte_cnt as _);
        for packed_ASCII_code(byte) in self.slice[1..byte_cnt].iter().copied() {
            val = fss_utf_acc_cont_byte(val, byte);
        }
        self.slice = &self.slice[byte_cnt..];
        Some(generalized_char::new(val))
    }
}

use crate::section_0038::packed_ASCII_code;
use core::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;
