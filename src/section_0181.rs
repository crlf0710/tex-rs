//! @ A global variable called |depth_threshold| is used to record the maximum
//! depth of nesting for which |show_node_list| will show information.  If we
//! have |depth_threshold=0|, for example, only the top level information will
//! be given and no sublists will be traversed. Another global variable, called
//! |breadth_max|, tells the maximum number of items to show at each level;
//! |breadth_max| had better be positive, or you won't see anything.
//
// @<Glob...@>=
// @!depth_threshold : integer; {maximum nesting depth in box displays}
/// maximum nesting depth in box displays
#[globals_struct_field(TeXGlobals)]
pub(crate) static depth_threshold: integer = 0;
// @!breadth_max : integer; {maximum number of items shown at the same list level}
/// maximum number of items shown at the same list level
#[globals_struct_field(TeXGlobals)]
pub(crate) static breadth_max: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};

