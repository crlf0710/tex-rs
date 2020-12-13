//! @ The next subroutine prints a whole glue specification.
//
// @p procedure print_spec(@!p:integer;@!s:str_number);
//   {prints a glue specification}
/// prints a glue specification
pub(crate) fn print_spec(globals: &mut TeXGlobals, p: integer, s: str_number) {
    // begin if (p<mem_min)or(p>=lo_mem_max) then print_char("*")
    if p < mem_min as integer || p >= globals.lo_mem_max as integer {
        print_char(make_globals_io_string_view!(globals), ASCII_code_literal!(b'*'));
    }
    // @.*\relax@>
    // else  begin print_scaled(width(p));
    else {
        print_scaled(globals, width!(globals, p as pointer));
        // if s<>0 then print(s);
        if s != 0 {
            print(globals, s.get() as _);
        }
        todo!();
        // if stretch(p)<>0 then
        //   begin print(" plus "); print_glue(stretch(p),stretch_order(p),s);
        //   end;
        // if shrink(p)<>0 then
        //   begin print(" minus "); print_glue(shrink(p),shrink_order(p),s);
        //   end;
        // end;
    }
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringView;
use crate::section_0011::mem_min;
use crate::section_0038::str_number;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0103::print_scaled;
use crate::section_0115::pointer;
