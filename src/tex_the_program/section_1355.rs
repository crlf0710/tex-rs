//! @ Each new type of node that appears in our data structure must be capable
//! of being displayed, copied, destroyed, and so on. The routines that we
//! need for write-oriented whatsits are somewhat like those for mark nodes;
//! other extensions might, of course, involve more subtlety here.
//
// @<Basic printing...@>=
// procedure print_write_whatsit(@!s:str_number;@!p:pointer);
pub(crate) fn print_write_whatsit(globals: &mut TeXGlobals, s: str_number, p: pointer) {
    // begin print_esc(s);
    print_esc(globals, s);
    // if write_stream(p)<16 then print_int(write_stream(p))
    if write_stream!(globals, p) < 16 {
        print_int(globals, write_stream!(globals, p) as _);
    }
    // else if write_stream(p)=16 then print_char("*")
    else if write_stream!(globals, p) == 16 {
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b'*'),
        );
    }
    // @.*\relax@>
    // else print_char("-");
    else {
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b'-'),
        );
    }
    // end;
}

use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code_literal;
use crate::section_0038::str_number;
use crate::section_0058::print_char;
use crate::section_0063::print_esc;
use crate::section_0065::print_int;
use crate::section_0115::pointer;
use crate::section_1341::write_stream;
