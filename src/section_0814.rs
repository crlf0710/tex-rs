//! @ The |line_break| procedure should be invoked only in horizontal mode; it
//! leaves that mode and places its output into the current vlist of the
//! enclosing vertical mode (or internal vertical mode).
//! There is one explicit parameter:  |final_widow_penalty| is the amount of
//! additional penalty to be inserted before the final line of the paragraph.
//!
//! There are also a number of implicit parameters: The hlist to be broken
//! starts at |link(head)|, and it is nonempty. The value of |prev_graf| in the
//! enclosing semantic level tells where the paragraph should begin in the
//! sequence of line numbers, in case hanging indentation or \.{\\parshape}
//! is in use; |prev_graf| is zero unless this paragraph is being continued
//! after a displayed formula.  Other implicit parameters, such as the
//! |par_shape_ptr| and various penalties to use for hyphenation, etc., appear
//! in |eqtb|.
//!
//! After |line_break| has acted, it will have updated the current vlist and the
//! value of |prev_graf|. Furthermore, the global variable |just_box| will
//! point to the final box created by |line_break|, so that the width of this
//! line can be ascertained when it is necessary to decide whether to use
//! |above_display_skip| or |above_display_short_skip| before a displayed formula.
//
// @<Glob...@>=
// @!just_box:pointer; {the |hlist_node| for the last line of the new paragraph}
/// the `hlist_node` for the last line of the new paragraph
#[globals_struct_field(TeXGlobals)]
pub(crate) static just_box: pointer = null;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::null;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
