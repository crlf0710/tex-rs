//! @ An |adjust_node|, which occurs only in horizontal lists,
//! specifies material that will be moved out into the surrounding
//! vertical list; i.e., it is used to implement \TeX's `\.{\\vadjust}'
//! operation.  The |adjust_ptr| field points to the vlist containing this
//! material.
//
// @d adjust_node=5 {|type| of an adjust node}
/// `type` of an adjust node
pub(crate) const adjust_node: quarterword = 5;
// @d adjust_ptr==mark_ptr {vertical list to be moved out of horizontal list}
/// vertical list to be moved out of horizontal list
pub(crate) macro adjust_ptr($globals:expr, $v:expr) {
    crate::section_0141::mark_ptr!($globals, $v)
}

use crate::section_0113::quarterword;
