//! @ Here is a procedure that displays the current command.
//
// @p procedure show_cur_cmd_chr;
#[allow(unused_variables)]
pub(crate) fn show_cur_cmd_chr(globals: &mut TeXGlobals) {
    // begin begin_diagnostic; print_nl("{");
    begin_diagnostic(globals);
    print_nl(globals, strpool_str!("{"));
    // if mode<>shown_mode then
    if mode!(globals) != globals.shown_mode.get() {
        // begin print_mode(mode); print(": "); shown_mode:=mode;
        print_mode(globals, (mode!(globals).get() as u16).into());
        print(globals, strpool_str!(": ").get() as _);
        globals.shown_mode = mode!(globals);
        // end;
    }
    // print_cmd_chr(cur_cmd,cur_chr); print_char("}");
    print_cmd_chr(globals, globals.cur_cmd, globals.cur_chr); 
    print_char(make_globals_io_string_view!(globals), ASCII_code_literal!(b'}'));
    // end_diagnostic(false);
    end_diagnostic(globals, false);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringView;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0062::print_nl;
use crate::section_0211::print_mode;
use crate::section_0245::begin_diagnostic;
use crate::section_0245::end_diagnostic;
use crate::section_0298::print_cmd_chr;
