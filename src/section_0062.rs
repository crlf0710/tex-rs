//! @ The procedure |print_nl| is like |print|, but it makes sure that the
//! string appears at the beginning of a new line.
//
// @<Basic print...@>=
// procedure print_nl(@!s:str_number); {prints string |s| at beginning of line}
/// prints string `s` at beginning of line
#[allow(unused_variables)]
pub(crate) fn print_nl(globals: &mut TeXGlobals, s: str_number) {
    // begin if ((term_offset>0)and(odd(selector)))or@|
    //   ((file_offset>0)and(selector>=log_only)) then print_ln;
    if globals.term_offset > 0 && globals.selector.is_odd()
        || globals.file_offset > 0 && globals.selector >= log_only
    {
        print_ln(make_globals_io_view!(globals));
    }
    // print(s);
    print(globals, s.into());
    // end;
}

use crate::pascal::IsOddOrEven;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0038::str_number;
use crate::section_0054::log_only;
use crate::section_0057::print_ln;
use crate::section_0059::print;
