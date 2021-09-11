//! @ In order to implement such an idea, \TeX\ maintains a stack of pointers
//! to the \\{down}, $y$, and $z$ commands that have been generated for the
//! current page. And there is a similar stack for \\{right}, |w|, and |x|
//! commands. These stacks are called the down stack and right stack, and their
//! top elements are maintained in the variables |down_ptr| and |right_ptr|.
//!
//! Each entry in these stacks contains four fields: The |width| field is
//! the amount of motion down or to the right; the |location| field is the
//! byte number of the \.{DVI} command in question (including the appropriate
//! |dvi_offset|); the |link| field points to the next item below this one
//! on the stack; and the |info| field encodes the options for possible change
//! in the \.{DVI} command.
//
// @d movement_node_size=3 {number of words per entry in the down and right stacks}
/// number of words per entry in the down and right stacks
pub(crate) const movement_node_size: quarterword = 3;
// @d location(#)==mem[#+2].int {\.{DVI} byte number for a movement command}
/// `DVI` byte number for a movement command
pub(crate) macro location($globals:expr, $ptr:expr) {
    $globals.mem[$ptr + 2][crate::section_0113::MEMORY_WORD_INT]
}

// @<Glob...@>=
// @!down_ptr,@!right_ptr:pointer; {heads of the down and right stacks}
/// heads of the down and right stacks
#[globals_struct_field(TeXGlobals)]
pub(crate) static down_ptr: pointer = null;
#[globals_struct_field(TeXGlobals)]
pub(crate) static right_ptr: pointer = null;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::null;

use crate::section_0004::TeXGlobals;
use crate::section_0113::quarterword;
use globals_struct::{globals_struct_field, globals_struct_use};
