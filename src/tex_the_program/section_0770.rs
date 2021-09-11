//! @ Alignments can occur within alignments, so a small stack is used to access
//! the alignrecord information. At each level we have a |preamble| pointer,
//! indicating the beginning of the preamble list; a |cur_align| pointer,
//! indicating the current position in the preamble list; a |cur_span| pointer,
//! indicating the value of |cur_align| at the beginning of a sequence of
//! spanned columns; a |cur_loop| pointer, indicating the tabskip glue before
//! an alignrecord that should be copied next if the current list is extended;
//! and the |align_state| variable, which indicates the nesting of braces so
//! that \.{\\cr} and \.{\\span} and tab marks are properly intercepted.
//! There also are pointers |cur_head| and |cur_tail| to the head and tail
//! of a list of adjustments being moved out from horizontal mode to
//! vertical~mode.
//!
//! The current values of these seven quantities appear in global variables;
//! when they have to be pushed down, they are stored in 5-word nodes, and
//! |align_ptr| points to the topmost such node.
//
// @d preamble==link(align_head) {the current preamble list}
/// the current preamble list
pub(crate) macro preamble($globals:expr) {
    crate::section_0118::link!($globals, crate::section_0162::align_head)
}
// @d align_stack_node_size=5 {number of |mem| words to save alignment states}
/// number of `mem` words to save alignment states
pub(crate) const align_stack_node_size: quarterword = 5;

// @<Glob...@>=
// @!cur_align:pointer; {current position in preamble list}
/// current position in preamble list
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_align: pointer = null;

// @!cur_span:pointer; {start of currently spanned columns in preamble list}
/// start of currently spanned columns in preamble list
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_span: pointer = null;
// @!cur_loop:pointer; {place to copy when extending a periodic preamble}
/// place to copy when extending a periodic preamble
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_loop: pointer = null;
// @!align_ptr:pointer; {most recently pushed-down alignment stack node}
/// most recently pushed-down alignment stack node
#[globals_struct_field(TeXGlobals)]
pub(crate) static align_ptr: pointer = null;

// @!cur_head,@!cur_tail:pointer; {adjustment list pointers}
/// adjustment list pointers
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_head: pointer = null;

/// adjustment list pointers
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_tail: pointer = null;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::null;

use crate::section_0113::quarterword;
use globals_struct::{globals_struct_field, globals_struct_use};
