//! @ The \TeX\ processor converts between ASCII code and
//! the user's external character set by means of arrays |xord| and |xchr|
//! that are analogous to \PASCAL's |ord| and |chr| functions.
//
// @<Glob...@>=
// @!xord: array [text_char] of ASCII_code;
//   {specifies conversion of input characters}
// @!xchr: array [ASCII_code] of text_char;
//   {specifies conversion of output characters}

#[cfg(not(feature = "unicode_support"))]
pub(crate) fn xord(val: text_char) -> ASCII_code {
    todo!();
}

#[cfg(feature = "unicode_support")]
pub(crate) fn xord(val: text_char) -> ASCII_code {
    ASCII_code(val.0)
}

#[cfg(not(feature = "unicode_support"))]
pub(crate) fn xchr(val: ASCII_code) -> text_char {
    todo!();
}

#[cfg(feature = "unicode_support")]
pub(crate) fn xchr(val: ASCII_code) -> text_char {
    text_char::new(val.0)
}

use crate::section_0018::ASCII_code;
use crate::section_0019::text_char;
