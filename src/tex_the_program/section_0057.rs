//! @ To end a line of text output, we call |print_ln|.
//
// @<Basic print...@>=
// procedure print_ln; {prints an end-of-line}

/// prints an end-of-line
#[allow(unused_variables)]
pub(crate) fn print_ln(mut globals: TeXGlobalsIoStringLogView<'_>) {
    // begin case selector of
    // term_and_log: begin wterm_cr; wlog_cr;
    if *globals.selector == term_and_log {
        wterm_cr(make_globals_io_view!(globals));
        wlog_cr(make_globals_log_view!(globals));
        // term_offset:=0; file_offset:=0;
        *globals.term_offset = 0.into();
        *globals.file_offset = 0.into();
    // end;
    }
    // log_only: begin wlog_cr; file_offset:=0;
    else if *globals.selector == log_only {
        wlog_cr(make_globals_log_view!(globals));
        *globals.file_offset = 0.into();
    // end;
    }
    // term_only: begin wterm_cr; term_offset:=0;
    else if *globals.selector == term_only {
        wterm_cr(make_globals_io_view!(globals));
        *globals.term_offset = 0.into();
    //   end;
    }
    // no_print,pseudo,new_string: do_nothing;
    else if *globals.selector == no_print
        || *globals.selector == pseudo
        || *globals.selector == new_string
    {
        do_nothing!();
    }
    // othercases write_ln(write_file[selector])
    else {
        write_ln_noargs(&mut globals.write_file[globals.selector.get()]);
    }
    // endcases;@/
    // end; {|tally| is not affected}
    /// `tally` is not affected
    const _: () = ();
}

use crate::pascal::write_ln_noargs;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0004::TeXGlobalsLogView;
use crate::section_0054::*;
use crate::section_0056::wlog_cr;
use crate::section_0056::wterm_cr;
