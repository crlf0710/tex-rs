//! @ \TeX\ is occasionally supposed to print diagnostic information that
//! goes only into the transcript file, unless |tracing_online| is positive.
//! Here are two routines that adjust the destination of print commands:
//
// @p procedure begin_diagnostic; {prepare to do some tracing}
/// prepare to do some tracing
#[allow(unused_variables)]
pub(crate) fn begin_diagnostic(globals: &mut TeXGlobals) {
    // begin old_setting:=selector;
    globals.diag_old_setting = globals.selector;
    // if (tracing_online<=0)and(selector=term_and_log) then
    if tracing_online!(globals) <= 0 && globals.selector == term_and_log {
        // begin decr(selector);
        decr!(globals.selector);
        // if history=spotless then history:=warning_issued;
        if globals.history == spotless {
            globals.history = warning_issued;
        }
        // end;
    }
    // end;
    use crate::section_0054::term_and_log;
    use crate::section_0076::history_kind::spotless;
    use crate::section_0076::history_kind::warning_issued;
}

// @#
// procedure end_diagnostic(@!blank_line:boolean);
//   {restore proper conditions after tracing}
/// restore proper conditions after tracing
#[allow(unused_variables)]
pub(crate) fn end_diagnostic(globals: &mut TeXGlobals, blank_line: boolean) {
    // begin print_nl("");
    print_nl(globals, crate::strpool_str!(""));
    // if blank_line then print_ln;
    if blank_line {
        print_ln(make_globals_io_string_log_view!(globals));
    }
    // selector:=old_setting;
    globals.selector = globals.diag_old_setting;
    // end;
}

use crate::pascal::boolean;
use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0016::decr;
use crate::section_0057::print_ln;
use crate::section_0062::print_nl;
use crate::section_0236::tracing_online;
