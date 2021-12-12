//! @ The outputs of |fetch| are placed in global variables.
//
// @<Glob...@>=
// @!cur_f:internal_font_number; {the |font| field of a |math_char|}
// @!cur_c:quarterword; {the |character| field of a |math_char|}
// @!cur_i:four_quarters; {the |char_info| of a |math_char|,
//   or a lig/kern instruction}
pub(crate) struct FetchedMathCharInfo {
    /// the `font` field of a `math_char`
    pub(crate) cur_f: internal_font_number,
    /// the `character` field of a `math_char`
    pub(crate) cur_c: ASCII_code,
    /// the `char_info` of a `math_char`, or a lig/kern instruction
    pub(crate) cur_i: char_info,
}

use crate::section_0018::ASCII_code;
use crate::section_0548::internal_font_number;
use crate::section_0554::char_info;
