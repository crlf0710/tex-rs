//! ` `
// @<Print string |s| on the terminal@>=
macro_rules! Print_string_s_on_the_terminal {
    ($globals:expr, $s:expr) => {{
        // begin if term_offset+length(s)>max_print_line-2 then print_ln
        if $globals.term_offset.get() as integer + length($globals, $s.get() as _) > 
            max_print_line as integer - 2 {
            print_ln(make_globals_io_view!($globals));
        }
        // else if (term_offset>0)or(file_offset>0) then print_char(" ");
        else if $globals.term_offset > 0 || $globals.file_offset > 0 {
            print_char(make_globals_io_string_view!($globals), ASCII_code_literal!(b' '));
        }
        // slow_print(s); update_terminal;
        slow_print($globals, $s.get() as _);
        update_terminal($globals);
        // end

        use crate::pascal::integer;
        use crate::section_0004::TeXGlobalsIoStringView;
        use crate::section_0004::TeXGlobalsIoView;
        use crate::section_0011::max_print_line;
        use crate::section_0034::update_terminal;        
        use crate::section_0040::length;
        use crate::section_0057::print_ln;
        use crate::section_0058::print_char;
        use crate::section_0060::slow_print;
    }}
}
