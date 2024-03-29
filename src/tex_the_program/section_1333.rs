//! @ Here we do whatever is needed to complete \TeX's job gracefully on the
//! local operating system. The code here might come into play after a fatal
//! error; it must therefore consist entirely of ``safe'' operations that
//! cannot produce error messages. For example, it would be a mistake to call
//! |str_room| or |make_string| at this time, because a call on |overflow|
//! might lead to an infinite loop.
//! @^system dependencies@>
//! (Actually there's one way to get error messages, via |prepare_mag|;
//! but that can't cause infinite recursion.)
//! @^recursion@>
//!
//! If |final_cleanup| is bypassed, this program doesn't bother to close
//! the input files that may still be open.

// @<Last-minute...@>=
// procedure close_files_and_terminate;
#[allow(unused_variables)]
pub(crate) fn close_files_and_terminate(globals: &mut TeXGlobals) {
    // var k:integer; {all-purpose index}
    // begin @<Finish the extensions@>; new_line_char:=-1;
    crate::section_1378::Finish_the_extensions!(globals);
    new_line_char!(globals) = -1;
    // @!stat if tracing_stats>0 then @<Output statistics about this job@>;@;@+tats@/
    crate::region_stat! {
        if tracing_stats!(globals) > 0 {
            crate::section_1334::Output_statistics_about_this_job!(globals);
        }
        use crate::section_0236::tracing_stats;
    }
    // wake_up_terminal; @<Finish the \.{DVI} file@>;
    wake_up_terminal(globals);
    crate::section_0642::Finish_the_DVI_file!(globals);
    // if log_opened then
    if globals.log_opened {
        // begin wlog_cr; a_close(log_file); selector:=selector-2;
        wlog_cr(make_globals_log_view!(globals));
        a_close(&mut globals.log_file);
        globals.selector = globals.selector - 2;
        // if selector=term_only then
        if globals.selector == term_only {
            // begin print_nl("Transcript written on ");
            print_nl(globals, crate::strpool_str!("Transcript written on "));
            // @.Transcript written...@>
            // slow_print(log_name); print_char(".");
            slow_print(globals, globals.log_name.get() as _);
            print_char(
                make_globals_io_string_log_view!(globals),
                ASCII_code_literal!(b'.'),
            );
            // end;
        }
        // end;
    }
    // end;
}

use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::make_globals_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0004::TeXGlobalsLogView;
use crate::section_0018::ASCII_code_literal;
use crate::section_0028::a_close;
use crate::section_0034::wake_up_terminal;
use crate::section_0054::term_only;
use crate::section_0056::wlog_cr;
use crate::section_0058::print_char;
use crate::section_0060::slow_print;
use crate::section_0062::print_nl;
use crate::section_0236::new_line_char;
