//! @ The reconstitution procedure shares many of the global data structures
//! by which \TeX\ has processed the words before they were hyphenated.
//! There is an implied ``cursor'' between characters |cur_l| and |cur_r|;
//! these characters will be tested for possible ligature activity. If
//! |ligature_present| then |cur_l| is a ligature character formed from the
//! original characters following |cur_q| in the current translation list.
//! There is a ``ligature stack'' between the cursor and character |j+1|,
//! consisting of pseudo-ligature nodes linked together by their |link| fields.
//! This stack is normally empty unless a ligature command has created a new
//! character that will need to be processed later. A pseudo-ligature is
//! a special node having a |character| field that represents a potential
//! ligature and a |lig_ptr| field that points to a |char_node| or is |null|.
//! We have
//! $$|cur_r|=\cases{|character(lig_stack)|,&if |lig_stack>null|;\cr
//!   |qi(hu[j+1])|,&if |lig_stack=null| and |j<n|;\cr
//!   bchar,&if |lig_stack=null| and |j=n|.\cr}$$
//
// @<Glob...@>=

// @!cur_l,@!cur_r:halfword; {characters before and after the cursor}
/// characters before and after the cursor
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_l: ASCII_code_or_non_char = ASCII_code_or_non_char::default();

#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_r: ASCII_code_or_non_char = ASCII_code_or_non_char::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0907::ASCII_code_or_non_char;

#[cfg(not(feature = "unicode_support"))]
pub(crate) type ASCII_code_or_non_char = halfword;

#[cfg(feature = "unicode_support")]
pub(crate) type ASCII_code_or_non_char = u32;

// @!cur_q:pointer; {where a ligature should be detached}
/// where a ligature should be detached
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_q: pointer = null;

// @!lig_stack:pointer; {unfinished business to the right of the cursor}
/// unfinished business to the right of the cursor
#[globals_struct_field(TeXGlobals)]
pub(crate) static lig_stack: pointer = null;

// @!ligature_present:boolean; {should a ligature node be made for |cur_l|?}
// @!lft_hit,@!rt_hit:boolean; {did we hit a ligature with a boundary character?}

use crate::section_0004::TeXGlobals;
use crate::section_0115::null;
use crate::section_0115::pointer;
use globals_struct::{globals_struct_field, globals_struct_use};
