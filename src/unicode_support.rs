type generalized_char = crate::pascal::char;

pub(crate) fn generalized_char_from_str(s: &str) -> generalized_char {
    runestr::rune::from_grapheme_cluster_lossy(s).expect("string is not a single grapheme cluster")
}

pub(crate) fn chars_from_generalized_char(val: generalized_char) -> impl Iterator<Item = char> {
    val.into_chars()
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
        panic!("info idx out of bound");
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

macro_rules! dump_four_bytes {
    ($globals:expr, $bytes:expr, $k:expr) => {{
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
    }};
}

pub(crate) fn dump_the_unicode_support_data(globals: &mut TeXGlobals) {
    dump_int!(globals, 0xABCDEF);
    let initial_value = runestr::internals::registry_rune_inner_initial();
    dump_int!(globals, initial_value as integer);
    let end_value = runestr::internals::registry_rune_inner_next();
    dump_int!(globals, (end_value - initial_value) as integer);
    for inner_value in initial_value..end_value {
        let rune = runestr::rune::from_inner(inner_value).unwrap();
        let mut str = runestr::RuneString::new();
        str.push(rune);
        let bytes = str.chars().collect::<String>().into_bytes();
        dump_int!(globals, bytes.len() as _);
        let mut idx = 0;
        while idx < bytes.len() {
            dump_four_bytes!(globals, bytes, idx);
            idx += 4;
        }
    }
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

macro_rules! undump_four_bytes {
    ($globals:expr, $bytes:expr, $k:expr) => {{
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
    }};
}

pub(crate) fn undump_the_unicode_support_data(globals: &mut TeXGlobals) -> bool {
    region_forward_label!(
    |'bad_fmt|
    {
        let mut x: integer;
        undump_int!(globals, x);
        if x != 0xABCDEF {
            goto_forward_label!('bad_fmt);
        }
        undump_int!(globals, x);
        let initial_value = runestr::internals::registry_rune_inner_initial();
        if x != initial_value as integer {
            goto_forward_label!('bad_fmt);
        }
        let count: integer;
        undump_int!(globals, count);
        if initial_value.checked_add(count as _).is_none() {
            return false;
        }
        let mut rune_next_value = initial_value;
        for _ in 0..count {
            let length: integer;
            undump_int!(globals, length);
            let mut bytes = vec![0; length as _];
            let mut idx = 0;
            while idx < bytes.len() {
                undump_four_bytes!(globals, bytes, idx);
                idx += 4;
            }
            let str = match std::str::from_utf8(&bytes) {
                Ok(s) => s,
                Err(_) => {
                    return false;
                }
            };
            let rune = match runestr::rune::from_grapheme_cluster_lossy(str) {
                Some(r) => r,
                None => {
                    return false;
                }
            };
            if rune.into_inner() != rune_next_value {
                return false;
            }
            rune_next_value += 1;
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
                rune_next_value,
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
                rune_next_value,
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
use core::cell::{Cell, RefCell};
use globals_struct::{globals_struct_field, globals_struct_use};
use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;
