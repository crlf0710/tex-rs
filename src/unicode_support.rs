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
    None
}

impl Iterator for GraphemeRegistryItemIter {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self {
            GraphemeRegistryItemIter::RegItem(item) => {
                match item {
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
                }
            }
            GraphemeRegistryItemIter::StrIter(iter) => {
                iter.next()
            }
            GraphemeRegistryItemIter::None => {
                None
            }
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

pub(crate) fn chars_from_generalized_char(val: generalized_char) -> impl Iterator<Item=char> {
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

use core::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;
