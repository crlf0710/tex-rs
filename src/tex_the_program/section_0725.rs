//! @ We need to do a lot of different things, so |mlist_to_hlist| makes two
//! passes over the given mlist.
//!
//! The first pass does most of the processing: It removes ``mu'' spacing from
//! glue, it recursively evaluates all subsidiary mlists so that only the
//! top-level mlist remains to be handled, it puts fractions and square roots
//! and such things into boxes, it attaches subscripts and superscripts, and
//! it computes the overall height and depth of the top-level mlist so that
//! the size of delimiters for a |left_noad| and a |right_noad| will be known.
//! The hlist resulting from each noad is recorded in that noad's |new_hlist|
//! field, an integer field that replaces the |nucleus| or |thickness|.
//! @^recursion@>
//!
//! The second pass eliminates all noads and inserts the correct glue and
//! penalties between nodes.
//
// @d new_hlist(#)==mem[nucleus(#)].int {the translation of an mlist}
/// the translation of an mlist
pub(crate) macro new_hlist($globals:expr, $v:expr) {
    $globals.mem[crate::section_0681::nucleus!($v)][crate::section_0113::MEMORY_WORD_INT]
}
