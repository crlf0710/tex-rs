//! @ The available-space list that keeps track of the variable-size portion
//! of |mem| is a nonempty, doubly-linked circular list of empty nodes,
//! pointed to by the roving pointer |rover|.
//!
//! Each empty node has size 2 or more; the first word contains the special
//! value |max_halfword| in its |link| field and the size in its |info| field;
//! the second word contains the two pointers for double linking.
//!
//! Each nonempty node also has size 2 or more. Its first word is of type
//! |two_halves|\kern-1pt, and its |link| field is never equal to |max_halfword|.
//! Otherwise there is complete flexibility with respect to the contents
//! of its other fields and its other words.
//!
//! (We require |mem_max<max_halfword| because terrible things can happen
//! when |max_halfword| appears in the |link| field of a nonempty node.)
//
// @d empty_flag == max_halfword {the |link| of an empty variable-size node}
/// the `link` of an empty variable-size node
pub(crate) const empty_flag: halfword = max_halfword;
// @d is_empty(#) == (link(#)=empty_flag) {tests for empty node}
/// tests for empty node
macro_rules! is_empty {
    ($globals:expr, $ptr:expr) => {
        link!($globals, $ptr) == crate::section_0124::empty_flag
    }
}
// @d node_size == info {the size field in empty variable-size nodes}
/// the size field in empty variable-size nodes
macro_rules! node_size {
    ($globals:expr, $ptr:expr) => {
        info_inner!($globals, $ptr)
    }
}
// @d llink(#) == info(#+1) {left link in doubly-linked list of empty nodes}
/// left link in doubly-linked list of empty nodes
macro_rules! llink {
    ($globals:expr, $ptr:expr) => {
        info_inner!($globals, $ptr + 1)
    }
}

// @d rlink(#) == link(#+1) {right link in doubly-linked list of empty nodes}
/// right link in doubly-linked list of empty nodes
macro_rules! rlink {
    ($globals:expr, $ptr:expr) => {
        link!($globals, $ptr + 1)
    }
}

// @<Glob...@>=
// @!rover : pointer; {points to some node in the list of empties}
/// points to some node in the list of empties
#[globals_struct_field(TeXGlobals)]
pub(crate) static rover: pointer = null;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

use crate::section_0004::TeXGlobals;
use crate::section_0110::max_halfword;
use crate::section_0113::halfword;
use globals_struct::{globals_struct_field, globals_struct_use};
