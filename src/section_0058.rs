//! @ The |print_char| procedure sends one character to the desired destination,
//! using the |xchr| array to map it into an external character compatible with
//! |input_ln|. All printing comes through |print_ln| or |print_char|.
//
// @<Basic printing...@>=
// procedure print_char(@!s:ASCII_code); {prints a single character}
/// prints a single character
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn print_char(mut globals: TeXGlobalsIoView<'_>, s: ASCII_code) {
    // label exit;
    // begin if @<Character |s| is the current new-line character@> then
    //  if selector<pseudo then
    //   begin print_ln; return;
    //   end;
    // case selector of
    // term_and_log: begin wterm(xchr[s]); wlog(xchr[s]);
    /*
    wterm(globals.reborrow(), xchr(s));
    wlog(globals.reborrow(), xchr(s));
    */
    //   incr(term_offset); incr(file_offset);
    //   if term_offset=max_print_line then
    //     begin wterm_cr; term_offset:=0;
    //     end;
    //   if file_offset=max_print_line then
    //     begin wlog_cr; file_offset:=0;
    //     end;
    //   end;
    // log_only: begin wlog(xchr[s]); incr(file_offset);
    //   if file_offset=max_print_line then print_ln;
    //   end;
    // term_only: begin wterm(xchr[s]); incr(term_offset);
    wterm(globals.reborrow(), xchr(s));
    incr!(*globals.term_offset);
    //   if term_offset=max_print_line then print_ln;
    //   end;
    // no_print: do_nothing;
    // pseudo: if tally<trick_count then trick_buf[tally mod error_line]:=s;
    // new_string: begin if pool_ptr<pool_size then append_char(s);
    //   end; {we drop characters if the string space is full}
    // othercases write(write_file[selector],xchr[s])
    // endcases;@/
    // incr(tally);
    // exit:end;
    //
}

use crate::section_0004::TeXGlobalsIoView;
use crate::section_0018::ASCII_code;
use crate::section_0020::xchr;
use crate::section_0056::wlog;
use crate::section_0056::wterm;
