//! ` `
// @<Declare the procedure called |restore_trace|@>=
// @!stat procedure restore_trace(@!p:pointer;@!s:str_number);
//   {|eqtb[p]| has just been restored or retained}
/// `eqtb[p]` has just been restored or retained
#[cfg(feature = "statistics")]
#[allow(unused_variables)]
pub(crate) fn restore_trace(globals: &mut TeXGlobals, p: pointer, s: str_number) {
    // begin begin_diagnostic; print_char("{"); print(s); print_char(" ");
    begin_diagnostic(globals);
    print_char(
        make_globals_io_string_log_view!(globals),
        ASCII_code_literal!(b'{'),
    );
    print(globals, s.get() as _);
    print_char(
        make_globals_io_string_log_view!(globals),
        ASCII_code_literal!(b' '),
    );
    // show_eqtb(p); print_char("}");
    show_eqtb(globals, p);
    print_char(
        make_globals_io_string_log_view!(globals),
        ASCII_code_literal!(b'}'),
    );
    // end_diagnostic(false);
    end_diagnostic(globals, false);
    // end;
    // tats
}

use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code_literal;
use crate::section_0038::str_number;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0115::pointer;
use crate::section_0245::begin_diagnostic;
use crate::section_0245::end_diagnostic;
#[cfg(feature = "statistics")]
use crate::section_0252::show_eqtb;
