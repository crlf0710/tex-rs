//! @ This routine should be changed, if necessary, to give the best possible
//! indication of where the current line resides in the input file.
//! For example, on some systems it is best to print both a page and line number.
//! @^system dependencies@>
//
// @<Print location of current line@>=
macro_rules! Print_location_of_current_line {
    ($globals:expr) => {{
        // if name<=17 then
        if name!($globals) <= 17 {
            todo!("Print location");
            //   if terminal_input then
            //     if base_ptr=0 then print_nl("<*>") else print_nl("<insert> ")
            //   else  begin print_nl("<read ");
            //     if name=17 then print_char("*")@+else print_int(name-1);
            // @.*\relax@>
            //     print_char(">");
            //     end
        }
        // else  begin print_nl("l."); print_int(line);
        else {
            print_nl($globals, strpool_str!("l."));
            print_int($globals, $globals.line);
            // end;
        }
        // print_char(" ")
        print_char(make_globals_io_string_view!($globals), ASCII_code_literal!(b' '));
        use crate::section_0004::TeXGlobalsIoStringView;
        use crate::section_0065::print_int;
        use crate::section_0058::print_char;
    }}
}
