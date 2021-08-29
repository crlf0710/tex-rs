//! @ When we skip conditional text, we keep track of the line number
//! where skipping began, for use in error messages.
//
// @<Glob...@>=
// @!skip_line:integer; {skipping began here}
/// skipping began here
#[globals_struct_field(TeXGlobals)]
pub(crate) static skip_line: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
