//! @ In case somebody has inadvertently made bad settings of the ``constants,''
//! \TeX\ checks them using a global variable called |bad|.
//!
//! This is the first of many sections of \TeX\ where global variables are
//! defined.
//
// @<Glob...@>=
// @!bad:integer; {is some ``constant'' wrong?}
/// is some "constant" wrong?
#[globals_struct_field(TeXGlobals)]
pub(crate) static bad: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
