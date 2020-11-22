//! @ Here now is the general |error| routine.
//
// @<Error hand...@>=
// procedure error; {completes the job of error reporting}
/// completes the job of error reporting
#[allow(unused_variables)]
pub(crate) fn error(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    todo!();
    // label continue,exit;
    // var c:ASCII_code; {what the user types}
    // @!s1,@!s2,@!s3,@!s4:integer;
    //   {used to save global variables when deleting tokens}
    // begin if history<error_message_issued then history:=error_message_issued;
    // print_char("."); show_context;
    // if interaction=error_stop_mode then @<Get user's advice and |return|@>;
    // incr(error_count);
    // if error_count=100 then
    //   begin print_nl("(That makes 100 errors; please try again.)");
    // @.That makes 100 errors...@>
    //   history:=fatal_error_stop; jump_out;
    //   end;
    // @<Put help message on the transcript file@>;
    // exit:end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::JumpOutToEndOfTEX;
