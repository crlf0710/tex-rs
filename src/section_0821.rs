//! @ The passive node for a given breakpoint contains only four fields:
//!
//! \yskip\hang|link| points to the passive node created just before this one,
//! if any, otherwise it is |null|.
//!
//! \yskip\hang|cur_break| points to the position of this breakpoint in the
//! horizontal list for the paragraph being broken.
//!
//! \yskip\hang|prev_break| points to the passive node that should precede this
//! one in an optimal path to this breakpoint.
//!
//! \yskip\hang|serial| is equal to |n| if this passive node is the |n|th
//! one created during the current pass. (This field is used only when
//! printing out detailed statistics about the line-breaking calculations.)
//!
//! \yskip\noindent
//! There is a global variable called |passive| that points to the most
//! recently created passive node. Another global variable, |printed_node|,
//! is used to help print out the paragraph when detailed information about
//! the line-breaking computation is being displayed.
//
// @d passive_node_size=2 {number of words in passive nodes}
// @d cur_break==rlink {in passive node, points to position of this breakpoint}
// @d prev_break==llink {points to passive node that should precede this one}
// @d serial==info {serial number for symbolic identification}
//
// @<Glob...@>=
// @!passive:pointer; {most recent node on passive list}

/// most recent node on passive list
#[globals_struct_field(TeXGlobals)]
pub(crate) static passive: pointer = null;
// @!printed_node:pointer; {most recent node that has been printed}
// @!pass_number:halfword; {the number of passive nodes allocated on this pass}

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
