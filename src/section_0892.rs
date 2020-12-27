//! @ The letters $c_1\ldots c_n$ that are candidates for hyphenation are placed
//! into an array called |hc|; the number |n| is placed into |hn|; pointers to
//! nodes $p_{a-1}$ and~$p_b$ in the description above are placed into variables
//! |ha| and |hb|; and the font number is placed into |hf|.
//
// @<Glob...@>=
// @!hc:array[0..65] of 0..256; {word to be hyphenated}
/// word to be hyphenated
#[globals_struct_field(TeXGlobals)]
pub(crate) static hc: [ASCII_code; 66] = [ASCII_code::default(); 66];
// @!hn:small_number; {the number of positions occupied in |hc|}
// @!ha,@!hb:pointer; {nodes |ha..hb| should be replaced by the hyphenated result}
// @!hf:internal_font_number; {font number of the letters in |hc|}
// @!hu:array[0..63] of 0..256; {like |hc|, before conversion to lowercase}
// @!hyf_char:integer; {hyphen character of the relevant font}
// @!cur_lang,@!init_cur_lang:ASCII_code; {current hyphenation table of interest}
/// current hyphenation table of interest
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_lang: ASCII_code = ASCII_code::default();

#[globals_struct_field(TeXGlobals)]
pub(crate) static init_cur_lang: ASCII_code = ASCII_code::default();
// @!l_hyf,@!r_hyf,@!init_l_hyf,@!init_r_hyf:integer; {limits on fragment sizes}
// @!hyf_bchar:halfword; {boundary character after $c_n$}

#[globals_struct_use(TeXGlobals)]
use crate::section_0018::ASCII_code;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
