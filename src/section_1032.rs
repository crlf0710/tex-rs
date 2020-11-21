//! @ The following part of the program was first written in a structured
//! manner, according to the philosophy that ``premature optimization is
//! the root of all evil.'' Then it was rearranged into pieces of
//! spaghetti so that the most common actions could proceed with little or
//! no redundancy.
//!
//! The original unoptimized form of this algorithm resembles the
//! |reconstitute| procedure, which was described earlier in connection with
//! hyphenation. Again we have an implied ``cursor'' between characters
//! |cur_l| and |cur_r|. The main difference is that the |lig_stack| can now
//! contain a charnode as well as pseudo-ligatures; that stack is now
//! usually nonempty, because the next character of input (if any) has been
//! appended to it. In |main_control| we have
//! $$|cur_r|=\cases{|character(lig_stack)|,&if |lig_stack>null|;\cr
//!   |font_bchar[cur_font]|,&otherwise;\cr}$$
//! except when |character(lig_stack)=font_false_bchar[cur_font]|.
//! Several additional global variables are needed.
//
// @<Glob...@>=
// @!main_f:internal_font_number; {the current font}
/// the current font
#[globals_struct_field(TeXGlobals)]
pub(crate) static main_f: internal_font_number = internal_font_number::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0548::internal_font_number;

// @!main_i:four_quarters; {character information bytes for |cur_l|}
// @!main_j:four_quarters; {ligature/kern command}
// @!main_k:font_index; {index into |font_info|}
/// index into `font_info`
#[globals_struct_field(TeXGlobals)]
pub(crate) static main_k: font_index = font_index::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0548::font_index;
// @!main_p:pointer; {temporary register for list manipulation}
// @!main_s:integer; {space factor value}
// @!bchar:halfword; {right boundary character of current font, or |non_char|}
// @!false_bchar:halfword; {nonexistent character matching |bchar|, or |non_char|}
// @!cancel_boundary:boolean; {should the left boundary be ignored?}
// @!ins_disc:boolean; {should we insert a discretionary node?}

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
