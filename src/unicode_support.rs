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

pub(crate) const info_val_count_limit: usize = 65536;

#[globals_struct_field(TeXGlobals)]
pub(crate) static info_val_gallery: Box<[cur_tok_repr]> = vec![0; info_val_count_limit].into();

#[globals_struct_use(TeXGlobals)]
use crate::unicode_support::info_val_count_limit;

#[globals_struct_field(TeXGlobals)]
pub(crate) static info_val_count: usize = 0;

#[globals_struct_use(TeXGlobals)]
use crate::section_0297::cur_tok_repr;

pub(crate) fn register_info_value(globals: &mut TeXGlobals, info_val: cur_tok_repr) -> halfword {
    for i in 0..globals.info_val_count {
        if globals.info_val_gallery[i] == info_val {
            return i as halfword;
        }
    }
    if globals.info_val_count + 1 >= info_val_count_limit {
        panic!("info idx used up");
    }
    let idx = globals.info_val_count;
    globals.info_val_gallery[idx] = info_val;
    globals.info_val_count += 1;
    idx as halfword
}

pub(crate) fn info_value(globals: &mut TeXGlobals, idx: halfword) -> cur_tok_repr {
    if idx as usize >= globals.info_val_count {
        panic!("info idx {} out of bound", idx);
    }
    globals.info_val_gallery[idx as usize]
}

pub(crate) const fontchar_val_count_limit: usize = 65536;

#[globals_struct_field(TeXGlobals)]
pub(crate) static fontchar_val_gallery: Box<[font_and_character]> =
    vec![font_and_character::default(); fontchar_val_count_limit].into();

#[globals_struct_use(TeXGlobals)]
use crate::unicode_support::fontchar_val_count_limit;

#[globals_struct_field(TeXGlobals)]
pub(crate) static fontchar_val_count: usize = 0;

#[globals_struct_use(TeXGlobals)]
use crate::section_0134::font_and_character;

pub(crate) fn register_fontchar_value(
    globals: &mut TeXGlobals,
    fontchar_val: font_and_character,
) -> halfword {
    for i in 0..globals.fontchar_val_count {
        if globals.fontchar_val_gallery[i] == fontchar_val {
            return i as halfword;
        }
    }
    if globals.fontchar_val_count + 1 >= fontchar_val_count_limit {
        panic!("fontchar idx used up");
    }
    let idx = globals.fontchar_val_count;
    globals.fontchar_val_gallery[idx] = fontchar_val;
    globals.fontchar_val_count += 1;
    idx as halfword
}

pub(crate) fn fontchar_value(globals: &mut TeXGlobals, idx: halfword) -> font_and_character {
    if idx as usize >= globals.fontchar_val_count {
        panic!("fontchar idx out of bound");
    }
    globals.fontchar_val_gallery[idx as usize]
}

#[globals_struct_field(TeXGlobals)]
pub(crate) static allow_big_char_code: boolean = false;

pub(crate) const triecharop_val_count_limit: usize = 65536;

#[globals_struct_field(TeXGlobals)]
pub(crate) static triecharop_val_gallery: Box<[trie_char_and_op]> =
    vec![trie_char_and_op::default(); triecharop_val_count_limit].into();

#[globals_struct_use(TeXGlobals)]
use crate::unicode_support::triecharop_val_count_limit;

#[globals_struct_field(TeXGlobals)]
pub(crate) static triecharop_val_count: usize = 0;

#[globals_struct_use(TeXGlobals)]
use crate::section_0921::trie_char_and_op;

pub(crate) fn register_triecharop_value(
    globals: &mut TeXGlobals,
    triecharop_val: trie_char_and_op,
) -> halfword {
    for i in 0..globals.triecharop_val_count {
        if globals.triecharop_val_gallery[i] == triecharop_val {
            return i as halfword;
        }
    }
    if globals.triecharop_val_count + 1 >= triecharop_val_count_limit {
        panic!("triecharop idx used up");
    }
    let idx = globals.triecharop_val_count;
    globals.triecharop_val_gallery[idx] = triecharop_val;
    globals.triecharop_val_count += 1;
    idx as halfword
}

pub(crate) fn triecharop_value(globals: &mut TeXGlobals, idx: halfword) -> trie_char_and_op {
    if idx as usize >= globals.triecharop_val_count {
        panic!("triecharop idx out of bound");
    }
    globals.triecharop_val_gallery[idx as usize]
}

pub(crate) macro dump_four_bytes($globals:expr, $bytes:expr, $k:expr) {{
    let mut w = four_quarters::default();
    w[FOUR_QUARTERS_B0] = *$bytes.get($k as usize).unwrap_or(&0);
    w[FOUR_QUARTERS_B1] = *$bytes.get($k as usize + 1).unwrap_or(&0);
    w[FOUR_QUARTERS_B2] = *$bytes.get($k as usize + 2).unwrap_or(&0);
    w[FOUR_QUARTERS_B3] = *$bytes.get($k as usize + 3).unwrap_or(&0);
    dump_qqqq!($globals, w);
    use crate::section_0113::four_quarters;
    use crate::section_0113::FOUR_QUARTERS_B0;
    use crate::section_0113::FOUR_QUARTERS_B1;
    use crate::section_0113::FOUR_QUARTERS_B2;
    use crate::section_0113::FOUR_QUARTERS_B3;
    use crate::section_1305::dump_qqqq;
}}

pub(crate) fn dump_the_unicode_support_data(globals: &mut TeXGlobals) {
    dump_int!(globals, 0xABCDEF);
    dump_int!(globals, GRAPHEME_REGISTRY_INITIAL_VALUE as integer);
    GRAPHE_REGISTRY.with(|reg| {
        let normalized_strings = reg.normalized_strings_after_single_scalar_value.borrow();
        assert!(
            reg.next_value.get() as integer
                == GRAPHEME_REGISTRY_INITIAL_VALUE as integer + normalized_strings.len() as integer
        );
        dump_int!(globals, normalized_strings.len() as _);
        for str in normalized_strings.iter() {
            let bytes = str.as_bytes();
            dump_int!(globals, bytes.len() as _);
            let mut idx = 0;
            while idx < bytes.len() {
                dump_four_bytes!(globals, bytes, idx);
                idx += 4;
            }
        }
    });
    dump_int!(globals, globals.info_val_count as _);
    for info_val in globals.info_val_gallery[..globals.info_val_count].iter() {
        dump_int!(globals, *info_val as _);
    }

    dump_int!(globals, globals.fontchar_val_count as _);
    for font_char_val in globals.fontchar_val_gallery[..globals.fontchar_val_count].iter() {
        dump_int!(globals, font_char_val.font.get() as _);
        dump_int!(globals, font_char_val.character.numeric_value() as _);
    }
    let allow_big_char_code = if globals.allow_big_char_code { 1 } else { 0 };
    dump_int!(globals, allow_big_char_code);

    dump_int!(globals, globals.triecharop_val_count as _);
    for triecharop_val in globals.triecharop_val_gallery[..globals.triecharop_val_count].iter() {
        dump_int!(globals, triecharop_val.char.numeric_value() as _);
        dump_int!(globals, triecharop_val.op as _);
    }
}

pub(crate) macro undump_four_bytes($globals:expr, $bytes:expr, $k:expr) {{
    let w;
    undump_qqqq!($globals, w);
    if let Some(byte) = $bytes.get_mut($k as usize) {
        *byte = w[FOUR_QUARTERS_B0];
    }
    if let Some(byte) = $bytes.get_mut($k as usize + 1) {
        *byte = w[FOUR_QUARTERS_B1];
    }
    if let Some(byte) = $bytes.get_mut($k as usize + 2) {
        *byte = w[FOUR_QUARTERS_B2];
    }
    if let Some(byte) = $bytes.get_mut($k as usize + 3) {
        *byte = w[FOUR_QUARTERS_B3];
    }

    use crate::section_0113::FOUR_QUARTERS_B0;
    use crate::section_0113::FOUR_QUARTERS_B1;
    use crate::section_0113::FOUR_QUARTERS_B2;
    use crate::section_0113::FOUR_QUARTERS_B3;
}}

pub(crate) fn undump_the_unicode_support_data(globals: &mut TeXGlobals) -> bool {
    crate::region_forward_label!(
    |'bad_fmt|
    {
        let mut x: integer;
        undump_int!(globals, x);
        if x != 0xABCDEF {
            crate::goto_forward_label!('bad_fmt);
        }
        undump_int!(globals, x);
        if x != GRAPHEME_REGISTRY_INITIAL_VALUE as integer {
            crate::goto_forward_label!('bad_fmt);
        }
        let mut graphme_registry_next_value = GRAPHEME_REGISTRY_INITIAL_VALUE;
        if !(GRAPHE_REGISTRY.with(|reg| -> bool {
            let count: integer;
            undump_int!(globals, count);
            if GRAPHEME_REGISTRY_INITIAL_VALUE.checked_add(count as _).is_none() {
                return false;
            }
            reg.normalized_strings_after_single_scalar_value.borrow_mut().clear();
            reg.normalized_string_lookup_map.borrow_mut().clear();
            reg.next_value.set(GRAPHEME_REGISTRY_INITIAL_VALUE);
            for _ in 0..count {
                let length: integer;
                undump_int!(globals, length);
                let mut bytes = vec![0; length as _];
                let mut idx = 0;
                while idx < bytes.len() {
                    undump_four_bytes!(globals, bytes, idx);
                    idx += 4;
                }
                let bytes = Box::leak(bytes.into_boxed_slice());
                let str = if let Ok(str) = core::str::from_utf8(bytes) {
                    str
                } else {
                    return false;
                };
                let cur_value = reg.next_value.get();
                reg.normalized_strings_after_single_scalar_value.borrow_mut().push(str);
                reg.normalized_string_lookup_map.borrow_mut().insert(str, cur_value);
                reg.next_value.set(cur_value + 1);
            }
            graphme_registry_next_value = reg.next_value.get();
            return true;
        })) {
            return false;
        }

        undump_size!(globals, 0, info_val_count_limit - 1, "info val count",
            globals.info_val_count, core::convert::identity, 'bad_fmt);
        for info_val in globals.info_val_gallery[..globals.info_val_count].iter_mut() {
            undump_int!(globals, *info_val, core::convert::identity);
        }

        undump_size!(globals, 0, fontchar_val_count_limit - 1, "fontchar val count",
            globals.fontchar_val_count, core::convert::identity, 'bad_fmt);
        for fontchar_val in globals.fontchar_val_gallery[..globals.fontchar_val_count].iter_mut() {
            undump!(
                globals,
                font_base,
                globals.font_ptr.get(),
                fontchar_val.font,
                internal_font_number::new,
                'bad_fmt
            );
            undump!(
                globals,
                0,
                graphme_registry_next_value,
                fontchar_val.character,
                ASCII_code::from_integer,
                'bad_fmt
            );
        }
        let allow_big_char_code;
        undump_int!(globals, allow_big_char_code);
        globals.allow_big_char_code = allow_big_char_code != 0;

        undump_size!(globals, 0, triecharop_val_count_limit - 1, "triecharop val count",
            globals.triecharop_val_count, core::convert::identity, 'bad_fmt);
        for triecharop_val in globals.triecharop_val_gallery[..globals.triecharop_val_count].iter_mut() {
            undump!(
                globals,
                0,
                graphme_registry_next_value,
                triecharop_val.char,
                ASCII_code::from_integer,
                'bad_fmt
            );
            undump!(
                globals,
                0,
                255,
                triecharop_val.op,
                core::convert::identity,
                'bad_fmt
            );
        }
        return true;
    }
    'bad_fmt <-
    );
    false
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0012::font_base;
use crate::section_0018::ASCII_code;
use crate::section_0038::packed_ASCII_code;
use crate::section_0113::halfword;
use crate::section_0134::font_and_character;
use crate::section_0297::cur_tok_repr;
use crate::section_0548::internal_font_number;
use crate::section_0921::trie_char_and_op;
use crate::section_1305::dump_int;
use crate::section_1306::undump;
use crate::section_1306::undump_int;
use crate::section_1306::undump_qqqq;
use crate::section_1306::undump_size;
use core::cell::{Cell, RefCell};
use globals_struct::{globals_struct_field, globals_struct_use};
use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;
