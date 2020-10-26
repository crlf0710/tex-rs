//! @ Here now is the first of the system-dependent routines for file name scanning.
//! @^system dependencies@>
//
// @p procedure begin_name;
#[allow(unused_variables)]
pub(crate) fn begin_name(globals: &mut TeXGlobals) {
    // begin area_delimiter:=0; ext_delimiter:=0;
    globals.area_delimiter = pool_pointer::new_zero();
    globals.ext_delimiter = pool_pointer::new_zero();
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::pool_pointer;
