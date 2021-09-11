//! ` `

// @<Declare subprocedures for |prefixed_command|@>=
// procedure new_interaction;
pub(crate) fn new_interaction(globals: &mut TeXGlobals) {
    // begin print_ln;
    print_ln(make_globals_io_string_log_view!(globals));
    // interaction:=cur_chr;
    globals.interaction = u8_from_m_to_n::new(globals.cur_chr.get() as _);
    // @<Initialize the print |selector| based on |interaction|@>;
    crate::section_0075::Initialize_the_print_selector_based_on_interaction!(globals);
    // if log_opened then selector:=selector+2;
    if globals.log_opened {
        globals.selector = globals.selector + 2;
    }
    // end;
}

use crate::pascal::u8_from_m_to_n;
use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0057::print_ln;
