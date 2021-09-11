//! @ To destroy the most recently made string, we say |flush_string|.
//
// @d flush_string==begin decr(str_ptr); pool_ptr:=str_start[str_ptr];
//   end
pub(crate) fn flush_string(globals: &mut TeXGlobals) {
    decr!(globals.str_ptr);
    globals.pool_ptr = globals.str_start[globals.str_ptr];
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
