//! ` `
// Here is a procedure that might be called `Get the next non-blank non-relax
// non-call non-assignment token'.
//
// @<Declare act...@>=
// procedure do_assignments;
pub(crate) fn do_assignments(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label exit;
    // begin loop begin @<Get the next non-blank non-relax...@>;
    loop {
        crate::section_0404::Get_the_next_non_blank_non_relax_non_call_token!(globals);
        // if cur_cmd<=max_non_prefixed_command then return;
        if globals.cur_cmd <= max_non_prefixed_command {
            crate::return_nojump!();
        }
        // set_box_allowed:=false; prefixed_command; set_box_allowed:=true;
        globals.set_box_allowed = false;
        prefixed_command(globals)?;
        globals.set_box_allowed = true;
        // end;
    }
    // exit:end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0208::max_non_prefixed_command;
use crate::section_1211::prefixed_command;
