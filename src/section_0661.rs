//! @ In order to provide a decent indication of where an overfull or underfull
//! box originated, we use a global variable |pack_begin_line| that is
//! set nonzero only when |hpack| is being called by the paragraph builder
//! or the alignment finishing routine.

//
// @<Glob...@>=
// @!pack_begin_line:integer; {source file line where the current paragraph
//   or alignment began; a negative value denotes alignment}
/// source file line where the current paragraph or alignment began;
/// a negative value denotes alignment
#[globals_struct_field(TeXGlobals)]
pub(crate) static pack_begin_line: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
