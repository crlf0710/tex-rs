//! ` `

// @<Glob...@>=
// @!save_stack : array[0..save_size] of memory_word;
// @!save_ptr : 0..save_size; {first unused entry on |save_stack|}
// @!max_save_stack:0..save_size; {maximum usage of save stack}
// @!cur_level: quarterword; {current nesting level for groups}
/// current nesting level for groups
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_level: quarterword = 0;
// @!cur_group: group_code; {current group type}
// @!cur_boundary: 0..save_size; {where the current level begins}

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
