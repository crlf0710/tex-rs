//! @ Here we do whatever is needed to complete \TeX's job gracefully on the
//! local operating system. The code here might come into play after a fatal
//! error; it must therefore consist entirely of ``safe'' operations that
//! cannot produce error messages. For example, it would be a mistake to call
//! |str_room| or |make_string| at this time, because a call on |overflow|
//! might lead to an infinite loop.
//! @^system dependencies@>
//!
//! Actually there's one way to get error messages, via |prepare_mag|;
//! but that can't cause infinite recursion.
//! @^recursion@>
//!
//! This program doesn't bother to close the input files that may still be open.

// @<Last-minute...@>=
// procedure close_files_and_terminate;
#[allow(unused_variables)]
pub(crate) fn close_files_and_terminate(globals: &mut TeXGlobals) {
    // var k:integer; {all-purpose index}
    // begin @<Finish the extensions@>;
    // @!stat if tracing_stats>0 then @<Output statistics about this job@>;@;@+tats@/
    // wake_up_terminal; @<Finish the \.{DVI} file@>;
    wake_up_terminal(globals);
    Finish_the_DVI_file!(globals);
    // if log_opened then
    if globals.log_opened {
        // begin wlog_cr; a_close(log_file); selector:=selector-2;
        wlog_cr(make_globals_log_view!(globals));
        a_close(&mut globals.log_file);
        globals.selector = globals.selector - 2;
        // if selector=term_only then
        if globals.selector == term_only {
            todo!();
            // begin print_nl("Transcript written on ");
            // @.Transcript written...@>
            // slow_print(log_name); print_char(".");
            // end;
        }
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsLogView;
use crate::section_0028::a_close;
use crate::section_0034::wake_up_terminal;
use crate::section_0054::term_only;
use crate::section_0056::wlog_cr;
