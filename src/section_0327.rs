//! @ The |back_error| routine is used when we want to replace an offending token
//! just before issuing an error message. This routine, like |back_input|,
//! requires that |cur_tok| has been set. We disable interrupts during the
//! call of |back_input| so that the help message won't be lost.


// @p procedure back_error; {back up one token and call |error|}
/// back up one token and call `error`
pub(crate) fn back_error(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    // begin OK_to_interrupt:=false; back_input; OK_to_interrupt:=true; error;
    globals.OK_to_interrupt = false;
    back_input(globals);
    globals.OK_to_interrupt = true;
    error(globals)?;
    // end;
    ok_nojump!()
}

// @#
// procedure ins_error; {back up one inserted token and call |error|}
// begin OK_to_interrupt:=false; back_input; token_type:=inserted;
// OK_to_interrupt:=true; error;
// end;
//

use crate::section_0004::TeXGlobals;
use crate::section_0081::JumpOutToEndOfTEX;
use crate::section_0082::error;
use crate::section_0325::back_input;

