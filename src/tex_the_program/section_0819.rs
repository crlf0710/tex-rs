//! @ An active node for a given breakpoint contains six fields:
//!
//! \yskip\hang|link| points to the next node in the list of active nodes; the
//! last active node has |link=last_active|.
//!
//! \yskip\hang|break_node| points to the passive node associated with this
//! breakpoint.
//!
//! \yskip\hang|line_number| is the number of the line that follows this
//! breakpoint.
//!
//! \yskip\hang|fitness| is the fitness classification of the line ending at this
//! breakpoint.
//!
//! \yskip\hang|type| is either |hyphenated| or |unhyphenated|, depending on
//! whether this breakpoint is a |disc_node|.
//!
//! \yskip\hang|total_demerits| is the minimum possible sum of demerits over all
//! lines leading from the beginning of the paragraph to this breakpoint.
//!
//! \yskip\noindent
//! The value of |link(active)| points to the first active node on a linked list
//! of all currently active nodes. This list is in order by |line_number|,
//! except that nodes with |line_number>easy_line| may be in any order relative
//! to each other.
//
// @d active_node_size=3 {number of words in active nodes}
/// number of words in active nodes
pub(crate) const active_node_size: quarterword = 3;
// @d fitness==subtype {|very_loose_fit..tight_fit| on final line for this break}
/// `very_loose_fit..tight_fit` on final line for this break
macro_rules! fitness {
    ($globals:expr, $p:expr) => {
        subtype!($globals, $p)
    }
}
// @d break_node==rlink {pointer to the corresponding passive node}
/// pointer to the corresponding passive node
macro_rules! break_node {
    ($globals:expr, $p:expr) => {
        rlink!($globals, $p)
    }
}
// @d line_number==llink {line that begins at this breakpoint}
/// line that begins at this breakpoint
macro_rules! line_number {
    ($globals:expr, $p:expr) => {
        llink!($globals, $p)
    }
}
// @d total_demerits(#)==mem[#+2].int {the quantity that \TeX\ minimizes}
/// the quantity that `TeX` minimizes
macro_rules! total_demerits {
    ($globals:expr, $p:expr) => {
        $globals.mem[$p + 2][crate::section_0113::MEMORY_WORD_INT]
    }
}
// @d unhyphenated=0 {the |type| of a normal active break node}
/// the `type` of a normal active break node
pub(crate) const unhyphenated: quarterword = 0;
// @d hyphenated=1 {the |type| of an active node that breaks at a |disc_node|}
/// the `type` of an active node that breaks at a `disc_node`
pub(crate) const hyphenated: quarterword = 1;
// @d last_active==active {the active list ends where it begins}
/// the active list ends where it begins
macro_rules! last_active {
    () => {
        crate::section_0162::active
    }
}

use crate::section_0113::quarterword;
