//! @ To end a line of text output, we call |print_ln|.
//
// @<Basic print...@>=
// procedure print_ln; {prints an end-of-line}

/// prints an end-of-line
#[allow(unused_variables)]
pub(crate) fn print_ln(globals: &mut TeXGlobals) {
    // begin case selector of
    // term_and_log: begin wterm_cr; wlog_cr;
    //   term_offset:=0; file_offset:=0;
    //   end;
    // log_only: begin wlog_cr; file_offset:=0;
    //   end;
    // term_only: begin wterm_cr; term_offset:=0;
    wterm_cr(make_globals_io_view!(globals));
    globals.term_offset = 0.into();
    //   end;
    // no_print,pseudo,new_string: do_nothing;
    // othercases write_ln(write_file[selector])
    // endcases;@/
    // end; {|tally| is not affected}
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0056::wterm_cr;
