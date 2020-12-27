//! @* \[41] Post-hyphenation.
//! If a hyphen may be inserted between |hc[j]| and |hc[j+1]|, the hyphenation
//! procedure will set |hyf[j]| to some small odd number. But before we look
//! at \TeX's hyphenation procedure, which is independent of the rest of the
//! line-breaking algorithm, let us consider what we will do with the hyphens
//! it finds, since it is better to work on this part of the program before
//! forgetting what |ha| and |hb|, etc., are all about.
//
// @<Glob...@>=
// @!hyf:array [0..64] of 0..9; {odd values indicate discretionary hyphens}
/// odd values indicate discretionary hyphens
#[globals_struct_field(TeXGlobals)]
pub(crate) static hyf: [u8_from_0_to_n<U9>; 65] = [u8_from_0_to_n::default(); 65];

// @!init_list:pointer; {list of punctuation characters preceding the word}
// @!init_lig:boolean; {does |init_list| represent a ligature?}
// @!init_lft:boolean; {if so, did the ligature involve a left boundary?}
//


#[globals_struct_use(TeXGlobals)]
use crate::pascal::u8_from_0_to_n;

#[globals_struct_use(TeXGlobals)]
use typenum::U9;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
