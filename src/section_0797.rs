//! @ A span node is a 2-word record containing |width|, |info|, and |link|
//! fields. The |link| field is not really a link, it indicates the number of
//! spanned columns; the |info| field points to a span node for the same
//! starting column, having a greater extent of spanning, or to |end_span|,
//! which has the largest possible |link| field; the |width| field holds the
//! largest natural width corresponding to a particular set of spanned columns.
//!
//! A list of the maximum widths so far, for spanned columns starting at a
//! given column, begins with the |info| field of the alignrecord for that
//! column.
//
// @d span_node_size=2 {number of |mem| words for a span node}
//
// @<Initialize the special list heads...@>=
macro_rules! Initialize_the_special_list_heads_and_constant_nodes_0797 {
    ($globals:expr) => {{
        // link(end_span):=max_quarterword+1; info(end_span):=null;
        link!($globals, end_span) = max_quarterword as pointer + 1;
        info_inner!($globals, end_span) = null;
        use crate::section_0110::max_quarterword;
        use crate::section_0115::pointer;
        use crate::section_0162::end_span;
    }}
}
