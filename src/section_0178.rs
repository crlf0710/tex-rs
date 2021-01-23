//! @ The next subroutine prints a whole glue specification.
//
// @p procedure print_spec(@!p:integer;@!s:str_number);
//   {prints a glue specification}
/// prints a glue specification
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn print_spec(globals: &mut TeXGlobals, p: integer, s: str_number) {
    // begin if (p<mem_min)or(p>=lo_mem_max) then print_char("*")
    if p < mem_min as integer || p >= globals.lo_mem_max as integer {
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b'*'),
        );
    }
    // @.*\relax@>
    // else  begin print_scaled(width(p));
    else {
        print_scaled(globals, width!(globals, p as pointer));
        // if s<>0 then print(s);
        if s != 0 {
            print(globals, s.get() as _);
        }
        // if stretch(p)<>0 then
        if stretch!(globals, p as pointer) != scaled::zero() {
            // begin print(" plus "); print_glue(stretch(p),stretch_order(p),s);
            print(globals, strpool_str!(" plus ").get() as _);
            print_glue(
                globals,
                stretch!(globals, p as pointer),
                stretch_order!(globals, p as pointer) as _,
                s,
            );
            // end;
        }
        // if shrink(p)<>0 then
        if shrink!(globals, p as pointer) != scaled::zero() {
            // begin print(" minus "); print_glue(shrink(p),shrink_order(p),s);
            print(globals, strpool_str!(" minus ").get() as _);
            print_glue(
                globals,
                shrink!(globals, p as pointer),
                shrink_order!(globals, p as pointer) as _,
                s,
            );
            // end;
        }
        // end;
    }
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0011::mem_min;
use crate::section_0038::str_number;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0101::scaled;
use crate::section_0103::print_scaled;
use crate::section_0115::pointer;
use crate::section_0177::print_glue;
