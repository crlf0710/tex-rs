//! ` `

// @d trie_link(#)==trie[#].rh {``downward'' link in a trie}
/// "downward" link in a trie
pub(crate) macro trie_link($globals:expr, $v:expr) {
    $globals.trie[$v][crate::section_0113::TWO_HALVES_RH]
}
// @d trie_char(#)==trie[#].b1 {character matched at this trie location}
/// character matched at this trie location
#[cfg(not(feature = "unicode_support"))]
#[allow(unused_macros)]
pub(crate) macro trie_char($globals:expr, $p:expr) {
    $globals.trie[$p as u16][TWO_HALVES_LH_B1]
}

/// character matched at this trie location
#[cfg(feature = "unicode_support")]
#[allow(unused_macros)]
pub(crate) macro trie_char($globals:expr, $p:expr) {
    crate::unicode_support::triecharop_value(
        $globals,
        $globals.trie[$p as u16][crate::section_0113::TWO_HALVES_LH],
    )
    .char
}

// @d trie_op(#)==trie[#].b0 {program for hyphenation at this trie location}
/// program for hyphenation at this trie location
#[cfg(not(feature = "unicode_support"))]
pub(crate) macro trie_op($globals:expr, $p:expr) {
    $globals.trie[$p][TWO_HALVES_LH_B0]
}

/// program for hyphenation at this trie location
#[cfg(feature = "unicode_support")]
pub(crate) macro trie_op($globals:expr, $p:expr) {
    crate::unicode_support::triecharop_value(
        $globals,
        $globals.trie[$p][crate::section_0113::TWO_HALVES_LH],
    )
    .op
}

#[cfg(not(feature = "unicode_support"))]
pub(crate) macro assign_trie_char_and_op($globals:expr, $p:expr, $char:expr, $op:expr) {{
    $globals.trie[$p][TWO_HALVES_LH_B1] = qi!($char);
    $globals.trie[$p][TWO_HALVES_LH_B0] = qi!($op);
}}

#[cfg(feature = "unicode_support")]
pub(crate) macro assign_trie_char_and_op($globals:expr, $p:expr, $char:expr, $op:expr) {{
    $globals.trie[$p][crate::section_0113::TWO_HALVES_LH] =
        crate::unicode_support::register_triecharop_value(
            $globals,
            crate::section_0921::trie_char_and_op {
                char: $char,
                op: $op,
            },
        );
}}

#[cfg(feature = "unicode_support")]
#[derive(Copy, Clone, PartialEq)]
pub(crate) struct trie_char_and_op {
    pub(crate) char: ASCII_code,
    pub(crate) op: quarterword,
}

#[cfg(feature = "unicode_support")]
impl Default for trie_char_and_op {
    fn default() -> Self {
        trie_char_and_op {
            char: ASCII_code_literal!(b' '),
            op: 0,
        }
    }
}

// @<Glob...@>=
// @!trie:array[trie_pointer] of two_halves; {|trie_link|, |trie_char|, |trie_op|}
/// `trie_link`, `trie_char`, `trie_op`
#[globals_struct_field(TeXGlobals)]
pub(crate) static trie: Box<trie_pointer_array<two_halves>> = Default::default();

// @!hyf_distance:array[1..trie_op_size] of small_number; {position |k-j| of $n_j$}
/// position `k-j` of `n_j`
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyf_distance: trie_op_val_array<small_number> = trie_op_val_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0943::trie_op_val_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0101::small_number;

// @!hyf_num:array[1..trie_op_size] of small_number; {value of $n_j$}
/// value of `n_j`
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyf_num: trie_op_val_array<small_number> = trie_op_val_array::default();

// @!hyf_next:array[1..trie_op_size] of quarterword; {continuation code}
/// continuation code
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyf_next: trie_op_val_array<quarterword> = trie_op_val_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::quarterword;

// @!op_start:array[ASCII_code] of 0..trie_op_size; {offset for current language}
/// offset for current language
#[globals_struct_field(TeXGlobals)]
pub(crate) static op_start: [u16_from_0_to_n<trie_op_size_TYPENUM>; 256] =
    [u16_from_0_to_n::default(); 256];

#[globals_struct_use(TeXGlobals)]
use crate::pascal::u16_from_0_to_n;

#[globals_struct_use(TeXGlobals)]
use crate::section_0011::trie_op_size_TYPENUM;

use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0018::ASCII_code_literal;
use crate::section_0113::quarterword;
use globals_struct::{globals_struct_field, globals_struct_use};
