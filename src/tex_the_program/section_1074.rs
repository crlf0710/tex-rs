//! @ The global variable |cur_box| will point to a newly made box. If the box
//! is void, we will have |cur_box=null|. Otherwise we will have
//! |type(cur_box)=hlist_node| or |vlist_node| or |rule_node|; the |rule_node|
//! case can occur only with leaders.
//
// @<Glob...@>=
// @!cur_box:pointer; {box to be placed into its context}
/// box to be placed into its context
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_box: pointer = null;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::null;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
