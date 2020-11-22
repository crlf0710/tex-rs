//! @ Here now is the (rather formidable) array of font arrays.
//
// @d non_char==qi(256) {a |halfword| code that can't match a real character}
#[cfg(not(feature = "unicode_support"))]
pub(crate) const non_char: ASCII_code_or_non_char = qi(256);
#[cfg(feature = "unicode_support")]
pub(crate) const non_char: ASCII_code_or_non_char = crate::pascal::char::MAX.0 + 1;
// @d non_address=0 {a spurious |bchar_label|}
/// a spurious `bchar_label`
pub(crate) const non_address: font_index = font_index::zero();

// @<Glob...@>=
// @!font_info:array[font_index] of memory_word;
//   {the big collection of font data}
#[globals_struct_field(TeXGlobals)]
pub(crate) static font_info: font_index_array<memory_word> = font_index_array::default_zeroed();

#[globals_struct_use(TeXGlobals)]
use crate::section_0549::font_index_array;

// @!fmem_ptr:font_index; {first unused word of |font_info|}
// @!font_ptr:internal_font_number; {largest internal font number in use}
// @!font_check:array[internal_font_number] of four_quarters; {check sum}
// @!font_size:array[internal_font_number] of scaled; {``at'' size}
// @!font_dsize:array[internal_font_number] of scaled; {``design'' size}
// @!font_params:array[internal_font_number] of font_index; {how many font
//   parameters are present}
// @!font_name:array[internal_font_number] of str_number; {name of the font}
// @!font_area:array[internal_font_number] of str_number; {area of the font}
// @!font_bc:array[internal_font_number] of eight_bits;
//   {beginning (smallest) character code}
// @!font_ec:array[internal_font_number] of eight_bits;
//   {ending (largest) character code}
// @!font_glue:array[internal_font_number] of pointer;
//   {glue specification for interword space, |null| if not allocated}
/// glue specification for interword space, |null| if not allocated
#[globals_struct_field(TeXGlobals)]
pub(crate) static font_glue: internal_font_array<pointer> = internal_font_array::default();

// @!font_used:array[internal_font_number] of boolean;
//   {has a character from this font actually appeared in the output?}
// @!hyphen_char:array[internal_font_number] of integer;
//   {current \.{\\hyphenchar} values}
// @!skew_char:array[internal_font_number] of integer;
//   {current \.{\\skewchar} values}
// @!bchar_label:array[internal_font_number] of font_index;
//   {start of |lig_kern| program for left boundary character,
//   |non_address| if there is none}
/// start of `lig_kern` program for left boundary character,
/// `non_address` if there is none
#[globals_struct_field(TeXGlobals)]
pub(crate) static bchar_label: internal_font_array<font_index> = internal_font_array::default();

// @!font_bchar:array[internal_font_number] of min_quarterword..non_char;
//   {right boundary character, |non_char| if there is none}
// @!font_false_bchar:array[internal_font_number] of min_quarterword..non_char;
//   {|font_bchar| if it doesn't exist in the font, otherwise |non_char|}

#[globals_struct_use(TeXGlobals)]
use crate::section_0549::internal_font_array;

type font_index_array_LENGTH_TYPENUM = typenum::op!(font_mem_size_TYPENUM - U0 + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) font_index_array[font_index] =>
    u16; U16; U0; font_index_array_LENGTH_TYPENUM
);

type internal_font_array_LENGTH_TYPENUM = typenum::op!(font_max_TYPENUM - font_base_TYPENUM + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) internal_font_array[internal_font_number] =>
    u16; U16; font_base_TYPENUM; internal_font_array_LENGTH_TYPENUM
);
use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0011::font_max_TYPENUM;
use crate::section_0011::font_mem_size_TYPENUM;
use crate::section_0012::font_base_TYPENUM;
use crate::section_0018::ASCII_code;
use crate::section_0113::memory_word;
use crate::section_0548::font_index;
use crate::section_0548::internal_font_number;
use crate::section_0907::ASCII_code_or_non_char;

use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U0;
use typenum::U1;

impl font_index_array<memory_word> {
    pub(crate) fn default_zeroed() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[allow(unused_variables)]
pub(crate) fn font_code_range_contains_char(
    globals: &mut TeXGlobals,
    font: internal_font_number,
    chr: ASCII_code,
) -> boolean {
    // FIXME: check with font_bc and font_ec
    false
}
