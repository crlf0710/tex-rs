//! @ Conditions can be inside conditions, and this nesting has a stack
//! that is independent of the |save_stack|.
//!
//! Four global variables represent the top of the condition stack:
//! |cond_ptr| points to pushed-down entries, if any; |if_limit| specifies
//! the largest code of a |fi_or_else| command that is syntactically legal;
//! |cur_if| is the name of the current type of conditional; and |if_line|
//! is the line number at which it began.
//!
//! If no conditions are currently in progress, the condition stack has the
//! special state |cond_ptr=null|, |if_limit=normal|, |cur_if=0|, |if_line=0|.
//! Otherwise |cond_ptr| points to a two-word node; the |type|, |subtype|, and
//! |link| fields of the first word contain |if_limit|, |cur_if|, and
//! |cond_ptr| at the next level, and the second word contains the
//! corresponding |if_line|.
//
// @d if_node_size=2 {number of words in stack entry for conditionals}
// @d if_line_field(#)==mem[#+1].int
// @d if_code=1 {code for \.{\\if...} being evaluated}
/// code for `\if...` being evaluated
pub(crate) const if_code: quarterword = 1;
// @d fi_code=2 {code for \.{\\fi}}
/// code for `\fi`
pub(crate) const fi_code: quarterword = 2;
// @d else_code=3 {code for \.{\\else}}
/// code for `\else`
pub(crate) const else_code: quarterword = 3;
// @d or_code=4 {code for \.{\\or}}
/// code for `\or`
pub(crate) const or_code: quarterword = 4;
pub(crate) type or_code_TYPENUM = U4;

// @<Glob...@>=
// @!cond_ptr:pointer; {top of the condition stack}
// @!if_limit:normal..or_code; {upper bound on |fi_or_else| codes}
/// upper bound on `fi_or_else` codes
#[globals_struct_field(TeXGlobals)]
pub(crate) static if_limit: u8_from_0_to_n<or_code_TYPENUM> = u8_from_0_to_n::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0489::or_code_TYPENUM;

// @!cur_if:small_number; {type of conditional being worked on}
// @!if_line:integer; {line where that conditional began}
//

use globals_struct::{globals_struct_field, globals_struct_use};
use crate::section_0004::TeXGlobals;
use crate::section_0113::quarterword;
use typenum::U4;
