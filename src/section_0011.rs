//! @ The following parameters can be changed at compile time to extend or
//! reduce \TeX's capacity. They may have different values in \.{INITEX} and
//! in production versions of \TeX.
//! @.INITEX@>
//! @^system dependencies@>

// @<Constants...@>=
// @!mem_max=30000; {greatest index in \TeX's internal |mem| array;
//   must be strictly less than |max_halfword|;
//   must be equal to |mem_top| in \.{INITEX}, otherwise |>=mem_top|}
// @!mem_min=0; {smallest index in \TeX's internal |mem| array;
//   must be |min_halfword| or more;
//   must be equal to |mem_bot| in \.{INITEX}, otherwise |<=mem_bot|}

// @!buf_size=500; {maximum number of characters simultaneously present in
//   current lines of open files and in control sequences between
//   \.{\\csname} and \.{\\endcsname}; must not exceed |max_halfword|}
/// maximum number of characters simultaneously present in
/// current lines of open files and in control sequences between
/// `\csname` and `\endcsname`; must not exceed `max_halfword`
pub(crate) const buf_size: u16 = buf_size_TYPENUM::U16;
pub(crate) type buf_size_TYPENUM = U500;

// @!error_line=72; {width of context lines on terminal error messages}
// @!half_error_line=42; {width of first lines of contexts in terminal
//   error messages; should be between 30 and |error_line-15|}
// @!max_print_line=79; {width of longest text lines output; should be at least 60}
// @!stack_size=200; {maximum number of simultaneous input sources}
// @!max_in_open=6; {maximum number of input files and error insertions that
//   can be going on simultaneously}
/// maximum number of input files and error insertions that can be going on simultaneously
pub(crate) type max_in_open_TYPENUM = U6;
pub(crate) const max_in_open: u8 = max_in_open_TYPENUM::U8;
// @!font_max=75; {maximum internal font number; must not exceed |max_quarterword|
//   and must be at most |font_base+256|}
// @!font_mem_size=20000; {number of words of |font_info| for all fonts}
// @!param_size=60; {maximum number of simultaneous macro parameters}
// @!nest_size=40; {maximum number of semantic levels simultaneously active}
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
// @!trie_size=8000; {space for hyphenation patterns; should be larger for
//   \.{INITEX} than it is in production versions of \TeX}
// @!trie_op_size=500; {space for ``opcodes'' in the hyphenation patterns}
// @!dvi_buf_size=800; {size of the output buffer; must be a multiple of 8}
// @!file_name_size=40; {file names shouldn't be longer than this}
// @!pool_name='TeXformats:TEX.POOL                     ';
//   {string of length |file_name_size|; tells where the string pool appears}
// @.TeXformats@>
//

use crate::section_0110::max_halfword;
type U3000 = ::typenum::op!(U1000 * U3);
type U32000 = ::typenum::op!(U10000 * U3 + U1000 * U2);
use static_assertions::const_assert;
use typenum::{Unsigned, U1000, U10000, U2, U3, U500, U6};
