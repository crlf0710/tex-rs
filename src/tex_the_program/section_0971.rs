//! @ A global variable |best_height_plus_depth| will be set to the natural size
//! of the box that corresponds to the optimum breakpoint found by |vert_break|.
//! (This value is used by the insertion-splitting algorithm of the page builder.)
//
// @<Glob...@>=
// @!best_height_plus_depth:scaled; {height of the best box, without stretching or
//   shrinking}
/// height of the best box, without stretching or shrinking
#[globals_struct_field(TeXGlobals)]
pub(crate) static best_height_plus_depth: scaled = scaled::zero();

#[globals_struct_use(TeXGlobals)]
use crate::section_0101::scaled;

use globals_struct::{globals_struct_field, globals_struct_use};
