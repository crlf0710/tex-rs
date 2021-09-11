//! @ Here now is the general |error| routine.
//
// @<Error hand...@>=
// procedure error; {completes the job of error reporting}
/// completes the job of error reporting
#[allow(unused_variables)]
pub(crate) fn error(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label continue,exit;
    // var c:ASCII_code; {what the user types}
    // @!s1,@!s2,@!s3,@!s4:integer;
    //   {used to save global variables when deleting tokens}
    // begin if history<error_message_issued then history:=error_message_issued;
    if globals.history < error_message_issued {
        globals.history = error_message_issued;
    }
    // print_char("."); show_context;
    print_char(
        make_globals_io_string_log_view!(globals),
        ASCII_code_literal!(b'.'),
    );
    show_context(globals);
    // if interaction=error_stop_mode then if selector<>log_only then
    //   @<Get user's advice and |return|@>;
    if globals.interaction == error_stop_mode && globals.selector != log_only {
        crate::section_0083::Get_user_s_advice_and_return!(globals);
    }
    // incr(error_count);
    incr!(globals.error_count);
    // if error_count=100 then
    if globals.error_count == 100 {
        // begin print_nl("(That makes 100 errors; please try again.)");
        print_nl(
            globals,
            crate::strpool_str!("(That makes 100 errors; please try again.)"),
        );
        // @.That makes 100 errors...@>
        // history:=fatal_error_stop; jump_out;
        globals.history = fatal_error_stop;
        jump_out()?;
        // end;
    }
    // @<Put help message on the transcript file@>;
    crate::section_0090::Put_help_message_on_the_transcript_file!(globals);
    // exit:end;
    crate::ok_nojump!()
}

use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0016::incr;
use crate::section_0018::ASCII_code_literal;
use crate::section_0054::log_only;
use crate::section_0058::print_char;
use crate::section_0062::print_nl;
use crate::section_0073::error_stop_mode;
use crate::section_0076::history_kind::error_message_issued;
use crate::section_0076::history_kind::fatal_error_stop;
use crate::section_0081::jump_out;
use crate::section_0081::TeXResult;
use crate::section_0311::show_context;
