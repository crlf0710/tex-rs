//! @ Then there is a subroutine that prints glue stretch and shrink, possibly
//! followed by the name of finite units:
//
// @p procedure print_glue(@!d:scaled;@!order:integer;@!s:str_number);
//   {prints a glue component}
/// prints a glue component
pub(crate) fn print_glue(globals: &mut TeXGlobals, d: scaled, mut order: integer, s: str_number) {
    // begin print_scaled(d);
    print_scaled(globals, d);
    // if (order<normal)or(order>filll) then print("foul")
    if order < glue_ord::normal as integer || order > glue_ord::filll as integer {
        print(globals, strpool_str!("foul").get() as _);
    }
    // else if order>normal then
    else if order > glue_ord::normal as integer {
        // begin print("fil");
        print(globals, strpool_str!("fil").get() as _);
        // while order>fil do
        while order > glue_ord::fil as integer {
            // begin print_char("l"); decr(order);
            print_char(
                make_globals_io_string_log_view!(globals),
                ASCII_code_literal!(b'l'),
            );
            decr!(order);
            // end;
        }
        // end
    }
    // else if s<>0 then print(s);
    else if s.get() != 0 {
        print(globals, s.get() as _);
    }
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0038::str_number;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0101::scaled;
use crate::section_0103::print_scaled;
use crate::section_0150::glue_ord;
