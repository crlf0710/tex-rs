//! @ The global variable |output_active| is true during the time the
//! user's output routine is driving \TeX.

//
// @<Glob...@>=
// @!output_active:boolean; {are we in the midst of an output routine?}
/// are we in the midst of an output routine?
#[globals_struct_field(TeXGlobals)]
pub(crate) static output_active: boolean = false;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
