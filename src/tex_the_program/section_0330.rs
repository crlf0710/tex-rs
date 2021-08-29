//! @ In order to keep the stack from overflowing during a long sequence of
//! inserted `\.{\\show}' commands, the following routine removes completed
//! error-inserted lines from memory.
//
// @p procedure clear_for_error_prompt;
#[allow(unused_variables)]
pub(crate) fn clear_for_error_prompt(globals: &mut TeXGlobals) {
    // begin while (state<>token_list)and terminal_input and@|
    //   (input_ptr>0)and(loc>limit) do end_file_reading;
    while state!(globals) != token_list && terminal_input(globals) &&
        globals.input_ptr > 0 && loc!(globals) > limit!(globals) {
        end_file_reading(globals);
    }
    // print_ln; clear_terminal;
    print_ln(make_globals_io_string_log_view!(globals));
    clear_terminal(globals);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0034::clear_terminal;
use crate::section_0057::print_ln;
use crate::section_0304::terminal_input;
use crate::section_0307::token_list;
use crate::section_0329::end_file_reading;