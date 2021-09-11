//! @ The |print_char| procedure sends one character to the desired destination,
//! using the |xchr| array to map it into an external character compatible with
//! |input_ln|. All printing comes through |print_ln| or |print_char|.
//
// @<Basic printing...@>=
// procedure print_char(@!s:ASCII_code); {prints a single character}
/// prints a single character
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn print_char(mut globals: TeXGlobalsIoStringLogView<'_>, s: ASCII_code) {
    // label exit;
    // begin if @<Character |s| is the current new-line character@> then
    if crate::section_0244::Character_s_is_the_current_new_line_character!(globals, s) {
        // if selector<pseudo then
        if *globals.selector < pseudo {
            // begin print_ln; return;
            print_ln(globals);
            return;
            // end;
        }
    }
    // case selector of
    // term_and_log: begin wterm(xchr[s]); wlog(xchr[s]);
    if *globals.selector == term_and_log {
        wterm(make_globals_io_view!(globals), xchr(s));
        wlog(make_globals_log_view!(globals), xchr(s));
        // incr(term_offset); incr(file_offset);
        incr!(*globals.term_offset);
        incr!(*globals.file_offset);
        // if term_offset=max_print_line then
        if *globals.term_offset == *globals.max_print_line {
            // begin wterm_cr; term_offset:=0;
            wterm_cr(make_globals_io_view!(globals));
            *globals.term_offset = 0.into();
            // end;
        }
        // if file_offset=max_print_line then
        if *globals.file_offset == *globals.max_print_line {
            // begin wlog_cr; file_offset:=0;
            wlog_cr(make_globals_log_view!(globals));
            *globals.file_offset = 0.into();
            // end;
        }
        // end;
    }
    // log_only: begin wlog(xchr[s]); incr(file_offset);
    else if *globals.selector == log_only {
        wlog(make_globals_log_view!(globals), xchr(s));
        incr!(*globals.file_offset);
        // if file_offset=max_print_line then print_ln;
        if *globals.file_offset == *globals.max_print_line {
            print_ln(globals.reborrow());
        }
        // end;
    }
    // term_only: begin wterm(xchr[s]); incr(term_offset);
    else if *globals.selector == term_only {
        wterm(make_globals_io_view!(globals), xchr(s));
        incr!(*globals.term_offset);
        // if term_offset=max_print_line then print_ln;
        if *globals.term_offset == *globals.max_print_line {
            print_ln(globals.reborrow());
        }
        // end;
    }
    // no_print: do_nothing;
    else if *globals.selector == no_print {
        do_nothing!();
    }
    // pseudo: if tally<trick_count then trick_buf[tally mod error_line]:=s;
    else if *globals.selector == pseudo {
        if globals.tally < globals.trick_count {
            globals.trick_buf[(*globals.tally % *globals.error_line as integer) as u8] = s;
        }
    }
    // new_string: begin if pool_ptr<pool_size then append_char(s);
    else if *globals.selector == new_string {
        if globals.pool_ptr.get() < pool_size as _ {
            append_char(make_globals_string_view!(globals), s);
        }
        // end; {we drop characters if the string space is full}
        /// we drop characters if the string space is full
        const _: () = ();
    }
    // othercases write(write_file[selector],xchr[s])
    else {
        write(&mut globals.write_file[globals.selector.get()], xchr(s));
    }
    // endcases;@/
    // incr(tally);
    incr!(*globals.tally);
    // exit:end;
}

use crate::pascal::integer;
use crate::pascal::write;
use crate::section_0004::make_globals_io_view;
use crate::section_0004::make_globals_log_view;
use crate::section_0004::make_globals_string_view;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0004::TeXGlobalsLogView;
use crate::section_0004::TeXGlobalsStringView;
use crate::section_0011::pool_size;
use crate::section_0016::do_nothing;
use crate::section_0016::incr;
use crate::section_0018::ASCII_code;
use crate::section_0020::xchr;
use crate::section_0042::append_char;
use crate::section_0054::log_only;
use crate::section_0054::new_string;
use crate::section_0054::no_print;
use crate::section_0054::pseudo;
use crate::section_0054::term_and_log;
use crate::section_0054::term_only;
use crate::section_0056::wlog;
use crate::section_0056::wlog_cr;
use crate::section_0056::wterm;
use crate::section_0056::wterm_cr;
use crate::section_0057::print_ln;
use crate::section_0115::pointer;
