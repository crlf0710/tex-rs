//! @ The page builder has another data structure to keep track of insertions.
//! This is a list of four-word nodes, starting and ending at |page_ins_head|.
//! That is, the first element of the list is node |r@t$_1$@>=link(page_ins_head)|;
//! node $r_j$ is followed by |r@t$_{j+1}$@>=link(r@t$_j$@>)|; and if there are
//! |n| items we have |r@t$_{n+1}$@>=page_ins_head|. The |subtype| field of
//! each node in this list refers to an insertion number; for example, `\.{\\insert
//! 250}' would correspond to a node whose |subtype| is |qi(250)|
//! (the same as the |subtype| field of the relevant |ins_node|). These |subtype|
//! fields are in increasing order, and |subtype(page_ins_head)=
//! qi(255)|, so |page_ins_head| serves as a convenient sentinel
//! at the end of the list. A record is present for each insertion number that
//! appears in the current page.
//!
//! The |type| field in these nodes distinguishes two possibilities that
//! might occur as we look ahead before deciding on the optimum page break.
//! If |type(r)=inserting|, then |height(r)| contains the total of the
//! height-plus-depth dimensions of the box and all its inserts seen so far.
//! If |type(r)=split_up|, then no more insertions will be made into this box,
//! because at least one previous insertion was too big to fit on the current
//! page; |broken_ptr(r)| points to the node where that insertion will be
//! split, if \TeX\ decides to split it, |broken_ins(r)| points to the
//! insertion node that was tentatively split, and |height(r)| includes also the
//! natural height plus depth of the part that would be split off.
//!
//! In both cases, |last_ins_ptr(r)| points to the last |ins_node|
//! encountered for box |qo(subtype(r))| that would be at least partially
//! inserted on the next page; and |best_ins_ptr(r)| points to the last
//! such |ins_node| that should actually be inserted, to get the page with
//! minimum badness among all page breaks considered so far. We have
//! |best_ins_ptr(r)=null| if and only if no insertion for this box should
//! be made to produce this optimum page.
//!
//! The data structure definitions here use the fact that the |@!height| field
//! appears in the fourth word of a box node.
//! @^data structure assumptions@>
//
// @d page_ins_node_size=4 {number of words for a page insertion node}
/// number of words for a page insertion node
pub(crate) const page_ins_node_size: quarterword = 4;
// @d inserting=0 {an insertion class that has not yet overflowed}
// @d split_up=1 {an overflowed insertion class}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub(crate) enum page_ins_node_subtype {
    /// an insertion class that has not yet overflowed
    inserting = 0,
    /// an overflowed insertion class
    split_up = 1,
}
// @d broken_ptr(#)==link(#+1)
//   {an insertion for this class will break here if anywhere}
// @d broken_ins(#)==info(#+1) {this insertion might break at |broken_ptr|}
/// this insertion might break at `broken_ptr`
macro_rules! broken_ins {
    ($globals:expr, $p:expr) => {
        info_inner!($globals, $p + 1)
    }
}

// @d last_ins_ptr(#)==link(#+2) {the most recent insertion for this |subtype|}
/// the most recent insertion for this `subtype`
macro_rules! last_ins_ptr {
    ($globals:expr, $p:expr) => {
        link!($globals, $p + 2)
    }
}

// @d best_ins_ptr(#)==info(#+2) {the optimum most recent insertion}
/// the optimum most recent insertion
macro_rules! best_ins_ptr {
    ($globals:expr, $p:expr) => {
        info_inner!($globals, $p + 2)
    }
}

// @<Initialize the special list heads...@>=
macro_rules! Initialize_the_special_list_heads_and_constant_nodes_0981 {
    ($globals:expr) => {{
        // subtype(page_ins_head):=qi(255);
        subtype!($globals, page_ins_head) = qi!(255);
        // type(page_ins_head):=split_up; link(page_ins_head):=page_ins_head;
        r#type!($globals, page_ins_head) = page_ins_node_subtype::split_up as _;
        link!($globals, page_ins_head) = page_ins_head;
        use crate::section_0162::page_ins_head;
        use crate::section_0981::page_ins_node_subtype;
    }}
}

use crate::section_0113::quarterword;