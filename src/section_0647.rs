//! @ If the global variable |adjust_tail| is non-null, the |hpack| routine
//! also removes all occurrences of |ins_node|, |mark_node|, and |adjust_node|
//! items and appends the resulting material onto the list that ends at
//! location |adjust_tail|.
//
// @< Glob...@>=
// @!adjust_tail:pointer; {tail of adjustment list}
/// tail of adjustment list
#[globals_struct_field(TeXGlobals)]
pub(crate) static adjust_tail: pointer = null;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::null;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
