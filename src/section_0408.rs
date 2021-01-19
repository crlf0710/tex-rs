//! @ Here is a procedure that sounds an alarm when mu and non-mu units
//! are being switched.
//
// @p procedure mu_error;
pub(crate) fn mu_error(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin print_err("Incompatible glue units");
    print_err!(globals, strpool_str!("Incompatible glue units"));
    // @.Incompatible glue units@>
    // help1("I'm going to assume that 1mu=1pt when they're mixed.");
    help1!(globals, strpool_str!("I'm going to assume that 1mu=1pt when they're mixed."));
    // error;
    error(globals)?;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0082::error;