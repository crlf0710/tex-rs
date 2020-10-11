//! @ The \TeX\ processor converts between ASCII code and
//! the user's external character set by means of arrays |xord| and |xchr|
//! that are analogous to \PASCAL's |ord| and |chr| functions.
//
// @<Glob...@>=
// @!xord: array [text_char] of ASCII_code;
//   {specifies conversion of input characters}
// @!xchr: array [ASCII_code] of text_char;
//   {specifies conversion of output characters}

#[allow(unused_variables)]
pub(crate) fn xord(val: text_char) -> ASCII_code {
    todo!();
}

use crate::section_0018::ASCII_code;
use crate::section_0019::text_char;
