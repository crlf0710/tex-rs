//! @ This routine should be changed, if necessary, to give the best possible
//! indication of where the current line resides in the input file.
//! For example, on some systems it is best to print both a page and line number.
//! @^system dependencies@>
//
// @<Print location of current line@>=
pub(crate) macro Print_location_of_current_line($globals:expr) {{
    // if name<=17 then
    if name!($globals) <= 17 {
        // if terminal_input then
        if terminal_input($globals) {
            // if base_ptr=0 then print_nl("<*>") else print_nl("<insert> ")
            if $globals.base_ptr == 0 {
                print_nl($globals, crate::strpool_str!("<*>"));
            } else {
                print_nl($globals, crate::strpool_str!("<insert> "));
            }
        }
        // else  begin print_nl("<read ");
        else {
            print_nl($globals, crate::strpool_str!("<read "));
            // if name=17 then print_char("*")@+else print_int(name-1);
            if name!($globals) == 17 {
                print_char(
                    make_globals_io_string_log_view!($globals),
                    ASCII_code_literal!(b'*'),
                );
            } else {
                print_int($globals, name!($globals) as integer - 1);
            }
            // @.*\relax@>
            // print_char(">");
            print_char(
                make_globals_io_string_log_view!($globals),
                ASCII_code_literal!(b'>'),
            );
            // end
        }
    }
    // else  begin print_nl("l."); print_int(line);
    else {
        print_nl($globals, crate::strpool_str!("l."));
        print_int($globals, $globals.line);
        // end;
    }
    // print_char(" ")
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b' '),
    );
    use crate::pascal::integer;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0062::print_nl;
    use crate::section_0065::print_int;
    use crate::section_0302::name;
    use crate::section_0304::terminal_input;
}}
