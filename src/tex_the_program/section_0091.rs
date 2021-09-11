//! @ A dozen or so error messages end with a parenthesized integer, so we
//! save a teeny bit of program space by declaring the following procedure:
//
// @p procedure int_error(@!n:integer);
pub(crate) fn int_error(globals: &mut TeXGlobals, n: integer) -> TeXResult<()> {
    // begin print(" ("); print_int(n); print_char(")"); error;
    print(globals, crate::strpool_str!(" (").get() as _);
    print_int(globals, n);
    print_char(
        make_globals_io_string_log_view!(globals),
        ASCII_code_literal!(b')'),
    );
    error(globals)?;
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0018::ASCII_code_literal;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0065::print_int;
use crate::section_0081::TeXResult;
use crate::section_0082::error;
