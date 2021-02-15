//! @* \[45] The page builder.
//! When \TeX\ appends new material to its main vlist in vertical mode, it uses
//! a method something like |vsplit| to decide where a page ends, except that
//! the calculations are done ``on line'' as new items come in.
//! The main complication in this process is that insertions must be put
//! into their boxes and removed from the vlist, in a more-or-less optimum manner.
//!
//! We shall use the term ``current page'' for that part of the main vlist that
//! is being considered as a candidate for being broken off and sent to the
//! user's output routine. The current page starts at |link(page_head)|, and
//! it ends at |page_tail|.  We have |page_head=page_tail| if this list is empty.
//! @^current page@>
//!
//! Utter chaos would reign if the user kept changing page specifications
//! while a page is being constructed, so the page builder keeps the pertinent
//! specifications frozen as soon as the page receives its first box or
//! insertion.  The global variable |page_contents| is |empty| when the
//! current page contains only mark nodes and content-less whatsit nodes; it
//! is |inserts_only| if the page contains only insertion nodes in addition to
//! marks and whatsits.  Glue nodes, kern nodes, and penalty nodes are
//! discarded until a box or rule node appears, at which time |page_contents|
//! changes to |box_there|.  As soon as |page_contents| becomes non-|empty|,
//! the current |vsize| and |max_depth| are squirreled away into |page_goal|
//! and |page_max_depth|; the latter values will be used until the page has
//! been forwarded to the user's output routine. The \.{\\topskip} adjustment
//! is made when |page_contents| changes to |box_there|.
//!
//! Although |page_goal| starts out equal to |vsize|, it is decreased by the
//! scaled natural height-plus-depth of the insertions considered so far, and by
//! the \.{\\skip} corrections for those insertions. Therefore it represents
//! the size into which the non-inserted material should fit, assuming that
//! all insertions in the current page have been made.
//!
//! The global variables |best_page_break| and |least_page_cost| correspond
//! respectively to the local variables |best_place| and |least_cost| in the
//! |vert_break| routine that we have already studied; i.e., they record the
//! location and value of the best place currently known for breaking the
//! current page. The value of |page_goal| at the time of the best break is
//! stored in |best_size|.
//
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub(crate) enum page_contents_kind {
    /// symbolic name for a null constant
    empty = 0,
    // @d inserts_only=1
    //   {|page_contents| when an insert node has been contributed, but no boxes}
    /// `page_contents` when an insert node has been contributed, but no boxes
    inserts_only = 1,
    // @d box_there=2 {|page_contents| when a box or rule has been contributed}
    /// `page_contents` when a box or rule has been contributed
    box_there = 2,
}

// @<Glob...@>=
// @!page_tail:pointer; {the final node on the current page}
/// the final node on the current page
#[globals_struct_field(TeXGlobals)]
pub(crate) static page_tail: pointer = null;
// @!page_contents:empty..box_there; {what is on the current page so far?}
/// what is on the current page so far?
#[globals_struct_field(TeXGlobals)]
pub(crate) static page_contents: page_contents_kind = page_contents_kind::empty;
// @!page_max_depth:scaled; {maximum box depth on page being built}
/// maximum box depth on page being built
#[globals_struct_field(TeXGlobals)]
pub(crate) static page_max_depth: scaled = scaled::zero();
// @!best_page_break:pointer; {break here to get the best page known so far}
// @!least_page_cost:integer; {the score for this currently best page}
// @!best_size:scaled; {its |page_goal|}

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

#[globals_struct_use(TeXGlobals)]
use crate::section_0980::page_contents_kind;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
