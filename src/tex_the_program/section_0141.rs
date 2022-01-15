//! @ A |mark_node| has a |mark_ptr| field that points to the reference count
//! of a token list that contains the user's \.{\\mark} text.
//! This field occupies a full word instead of a halfword, because
//! there's nothing to put in the other halfword; it is easier in \PASCAL\ to
//! use the full word than to risk leaving garbage in the unused half.
//
// @d mark_node=4 {|type| of a mark node}
/// `type` of a mark node
pub(crate) const mark_node: quarterword = 4;
// @d small_node_size=2 {number of words to allocate for most node types}
/// number of words to allocate for most node types
pub(crate) const small_node_size: quarterword = 2;
// @d mark_ptr(#)==mem[#+1].int {head of the token list for a mark}
/// head of the token list for a mark
pub(crate) macro mark_ptr($globals:expr, $v:expr) {{
    $globals.mem[$v + 1][crate::section_0113::MEMORY_WORD_INT]
}}

use crate::section_0113::quarterword;
