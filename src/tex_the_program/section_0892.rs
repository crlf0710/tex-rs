//! @ The letters $c_1\ldots c_n$ that are candidates for hyphenation are placed
//! into an array called |hc|; the number |n| is placed into |hn|; pointers to
//! nodes $p_{a-1}$ and~$p_b$ in the description above are placed into variables
//! |ha| and |hb|; and the font number is placed into |hf|.
//
// @<Glob...@>=
// @!hc:array[0..65] of 0..256; {word to be hyphenated}
/// word to be hyphenated
#[globals_struct_field(TeXGlobals)]
pub(crate) static hc: [ASCII_code_or_non_char; 66] = [ASCII_code_or_non_char::default(); 66];
// @!hn:0..64; {the number of positions occupied in |hc|;
//                                   not always a |small_number|}
/// the number of positions occupied in `hc`; not always a `small_number`
#[globals_struct_field(TeXGlobals)]
pub(crate) static hn: u8_from_0_to_n<U64> = u8_from_0_to_n::default();
// @!ha,@!hb:pointer; {nodes |ha..hb| should be replaced by the hyphenated result}
/// nodes `ha..hb` should be replaced by the hyphenated result
#[globals_struct_field(TeXGlobals)]
pub(crate) static ha: pointer = null;
#[globals_struct_field(TeXGlobals)]
pub(crate) static hb: pointer = null;
// @!hf:internal_font_number; {font number of the letters in |hc|}
/// font number of the letters in `hc`
#[globals_struct_field(TeXGlobals)]
pub(crate) static hf: internal_font_number = internal_font_number::default();
// @!hu:array[0..63] of 0..256; {like |hc|, before conversion to lowercase}
/// like `hc`, before conversion to lowercase
#[globals_struct_field(TeXGlobals)]
pub(crate) static hu: [ASCII_code_or_non_char; 64] = [ASCII_code_or_non_char::default(); 64];
// @!hyf_char:integer; {hyphen character of the relevant font}
/// hyphen character of the relevant font
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyf_char: integer = 0;
// @!cur_lang,@!init_cur_lang:ASCII_code; {current hyphenation table of interest}
/// current hyphenation table of interest
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_lang: ASCII_code = ASCII_code::default();

#[globals_struct_field(TeXGlobals)]
pub(crate) static init_cur_lang: ASCII_code = ASCII_code::default();
// @!l_hyf,@!r_hyf,@!init_l_hyf,@!init_r_hyf:integer; {limits on fragment sizes}
/// limits on fragment sizes
#[globals_struct_field(TeXGlobals)]
pub(crate) static l_hyf: integer = 0;
#[globals_struct_field(TeXGlobals)]
pub(crate) static r_hyf: integer = 0;
#[globals_struct_field(TeXGlobals)]
pub(crate) static init_l_hyf: integer = 0;
#[globals_struct_field(TeXGlobals)]
pub(crate) static init_r_hyf: integer = 0;

// @!hyf_bchar:halfword; {boundary character after $c_n$}
/// boundary character after `c_n`
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyf_bchar: ASCII_code_or_non_char = ASCII_code_or_non_char::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0018::ASCII_code;

#[globals_struct_use(TeXGlobals)]
use typenum::U64;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};

use super::section_0907::ASCII_code_or_non_char;
