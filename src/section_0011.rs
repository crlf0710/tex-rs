//! @ The following parameters can be changed at compile time to extend or
//! reduce \TeX's capacity. They may have different values in \.{INITEX} and
//! in production versions of \TeX.
//! @.INITEX@>
//! @^system dependencies@>

// @<Constants...@>=
// @!mem_max=30000; {greatest index in \TeX's internal |mem| array;
//   must be strictly less than |max_halfword|;
//   must be equal to |mem_top| in \.{INITEX}, otherwise |>=mem_top|}
/// greatest index in TeX's internal mem array; must be strictly less
/// than `max_halfword`; must be equal to mem top in INITEX,
/// otherwise `>= mem_top`
pub(crate) const mem_max: u32 = mem_max_TYPENUM::U32;
pub(crate) type mem_max_TYPENUM = U30000;

// @!mem_min=0; {smallest index in \TeX's internal |mem| array;
//   must be |min_halfword| or more;
//   must be equal to |mem_bot| in \.{INITEX}, otherwise |<=mem_bot|}
/// smallest index in TeX's internal `mem` array; must be `min_halfword`
/// or more; must be equal to `mem_bot` in `INITEX`, otherwise `<=mem_bot`
pub(crate) const mem_min: u32 = mem_max_TYPENUM::U32;
pub(crate) type mem_min_TYPENUM = U0;

// @!buf_size=500; {maximum number of characters simultaneously present in
//   current lines of open files and in control sequences between
//   \.{\\csname} and \.{\\endcsname}; must not exceed |max_halfword|}
/// maximum number of characters simultaneously present in
/// current lines of open files and in control sequences between
/// `\csname` and `\endcsname`; must not exceed `max_halfword`
pub(crate) const buf_size: u16 = buf_size_TYPENUM::U16;
pub(crate) type buf_size_TYPENUM = U500;

// @!error_line=72; {width of context lines on terminal error messages}
/// width of context lines on terminal error messages
#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringLogView)]
pub(crate) static error_line: quarterword = 72;

// @!half_error_line=42; {width of first lines of contexts in terminal
//   error messages; should be between 30 and |error_line-15|}
/// width of first lines of contexts in terminal error messages;
/// should be between 30 and `error_line-15`
#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringLogView)]
pub(crate) static half_error_line: quarterword = 42;
// @!max_print_line=79; {width of longest text lines output; should be at least 60}
/// width of longest text lines output; should be at least 60
#[globals_struct_field(TeXGlobals)]
#[globals_struct_field_view(TeXGlobalsIoStringLogView)]
pub(crate) static max_print_line: quarterword = 79;
// @!stack_size=200; {maximum number of simultaneous input sources}
/// maximum number of simultaneous input sources
pub(crate) const stack_size: u16 = stack_size_TYPENUM::U16;
pub(crate) type stack_size_TYPENUM = U200;
// @!max_in_open=6; {maximum number of input files and error insertions that
//   can be going on simultaneously}
/// maximum number of input files and error insertions that can be going on simultaneously
pub(crate) const max_in_open: u8 = max_in_open_TYPENUM::U8;
pub(crate) type max_in_open_TYPENUM = U6;
// @!font_max=75; {maximum internal font number; must not exceed |max_quarterword|
//   and must be at most |font_base+256|}
pub(crate) const font_max: u8 = font_max_TYPENUM::U8;
pub(crate) type font_max_TYPENUM = U75;
// @!font_mem_size=20000; {number of words of |font_info| for all fonts}
/// number of words of `font_info` for all fonts
pub(crate) const font_mem_size: u16 = font_mem_size_TYPENUM::U16;
pub(crate) type font_mem_size_TYPENUM = U20000;
// @!param_size=60; {maximum number of simultaneous macro parameters}
/// maximum number of simultaneous macro parameters
pub(crate) const param_size: u8 = param_size_TYPENUM::U8;
pub(crate) type param_size_TYPENUM = U60;
// @!nest_size=40; {maximum number of semantic levels simultaneously active}
/// maximum number of semantic levels simultaneously active
pub(crate) const nest_size: u8 = nest_size_TYPENUM::U8;
pub(crate) type nest_size_TYPENUM = U40;
// @!max_strings=3000; {maximum number of strings; must not exceed |max_halfword|}

/// maximum number of strings; must not exceed `max_halfword`
pub(crate) const max_strings: u16 = max_strings_TYPENUM::U16;
pub(crate) type max_strings_TYPENUM = U3000;
const_assert!(max_strings as u32 <= max_halfword as u32);

// @!string_vacancies=8000; {the minimum number of characters that should be
//   available for the user's control sequences and font names,
//   after \TeX's own error messages are stored}
// @!pool_size=32000; {maximum number of characters in strings, including all
//   error messages and help texts, and the names of all fonts and
//   control sequences; must exceed |string_vacancies| by the total
//   length of \TeX's own strings, which is currently about 23000}
/// maximum number of characters in strings, including all
///   error messages and help texts, and the names of all fonts and
///   control sequences; must exceed |string_vacancies| by the total
///   length of \TeX's own strings, which is currently about 23000
pub(crate) const pool_size: u16 = pool_size_TYPENUM::U16;
pub(crate) type pool_size_TYPENUM = U32000;
// @!save_size=600; {space for saving values outside of current group; must be
//   at most |max_halfword|}
/// space for saving values outside of current group; must be
///   at most `max_halfword`
pub(crate) const save_size: u16 = save_size_TYPENUM::U16;
pub(crate) type save_size_TYPENUM = U600;
// @!trie_size=8000; {space for hyphenation patterns; should be larger for
//   \.{INITEX} than it is in production versions of \TeX}
/// space for hyphenation patterns; should be larger for `INITEX`
/// than it is in production versions of `TeX`
pub(crate) const trie_size: u16 = trie_size_TYPENUM::U16;
pub(crate) type trie_size_TYPENUM = U8000;
// @!trie_op_size=500; {space for ``opcodes'' in the hyphenation patterns}
/// space for "opcodes" in the hyphenation patterns
pub(crate) const trie_op_size: u16 = trie_op_size_TYPENUM::U16;
pub(crate) type trie_op_size_TYPENUM = U500;
pub(crate) type trie_op_size_POS_TYPENUM = P500;
pub(crate) type trie_op_size_NEG_TYPENUM = N500;

// @!dvi_buf_size=800; {size of the output buffer; must be a multiple of 8}
// @!file_name_size=40; {file names shouldn't be longer than this}
/// file names shouldn't be longer than this
pub(crate) const file_name_size: u16 = file_name_size_TYPENUM::U16;
pub(crate) type file_name_size_TYPENUM = U40;
// @!pool_name='TeXformats:TEX.POOL                     ';
//   {string of length |file_name_size|; tells where the string pool appears}
/// string of length `file_name_size`; tells where the string pool appears
pub(crate) const pool_name: &'static str = "TeXformats:TEX.POOL                     ";
// @.TeXformats@>
//

#[globals_struct_use(TeXGlobals)]
use crate::section_0113::quarterword;

use crate::section_0110::max_halfword;
type U3000 = ::typenum::op!(U1000 * U3);
type U8000 = ::typenum::op!(U1000 * U8);
type U20000 = ::typenum::op!(U10000 * U2);
type U30000 = ::typenum::op!(U10000 * U3);
type U32000 = ::typenum::op!(U10000 * U3 + U1000 * U2);
use static_assertions::const_assert;
use typenum::{
    Unsigned, U0, U1000, U10000, U2, U200, U3, U40, U42, U500, U6, U60, U600, U72, U75, U79, U8
};
use typenum::{N500, P500};
use globals_struct::{globals_struct_field, globals_struct_use};