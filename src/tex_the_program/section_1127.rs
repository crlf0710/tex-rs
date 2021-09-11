//! ` `
// @<Declare act...@>=
// procedure align_error;
pub(crate) fn align_error(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin if abs(align_state)>2 then
    if globals.align_state.abs() > 2 {
        // @<Express consternation over the fact that no alignment is in progress@>
        crate::section_1128::Express_consternation_over_the_fact_that_no_alignment_is_in_progress!(
            globals
        );
    }
    // else  begin back_input;
    else {
        back_input(globals);
        // if align_state<0 then
        if globals.align_state < 0 {
            todo!("Missing {{");
            //   begin print_err("Missing { inserted");
            // @.Missing \{ inserted@>
            //   incr(align_state); cur_tok:=left_brace_token+"{";
            //   end
        }
        // else  begin print_err("Missing } inserted");
        else {
            todo!("Missing }}");
            // @.Missing \} inserted@>
            //   decr(align_state); cur_tok:=right_brace_token+"}";
            //   end;
        }
        // help3("I've put in what seems to be necessary to fix")@/
        //   ("the current column of the current alignment.")@/
        //   ("Try to go on, since this might almost work."); ins_error;
        help3!(
            globals,
            crate::strpool_str!("I've put in what seems to be necessary to fix"),
            crate::strpool_str!("the current column of the current alignment."),
            crate::strpool_str!("Try to go on, since this might almost work.")
        );
        ins_error(globals)?;
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0079::help3;
use crate::section_0081::TeXResult;
use crate::section_0325::back_input;
use crate::section_0327::ins_error;
