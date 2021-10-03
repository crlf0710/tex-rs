//! ` `
// @<Print a symbolic description of the new break node@>=
#[cfg(feature = "statistics")]
pub(crate) macro Print_a_symbolic_description_of_the_new_break_node($globals:expr, $q:expr, $break_type:expr, $fit_class:expr) {{
    // begin print_nl("@@@@"); print_int(serial(passive));
    print_nl($globals, crate::strpool_str!("@@"));
    print_int($globals, serial!($globals, $globals.passive) as _);
    // @.\AT!\AT!@>
    // print(": line "); print_int(line_number(q)-1);
    print($globals, crate::strpool_str!(": line ").get() as _);
    print_int($globals, line_number!($globals, $q) as integer - 1);
    // print_char("."); print_int(fit_class);
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b'.'),
    );
    print_int($globals, $fit_class as _);
    // if break_type=hyphenated then print_char("-");
    if $break_type == hyphenated {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'-'),
        );
    }
    // print(" t="); print_int(total_demerits(q));
    print($globals, crate::strpool_str!(" t=").get() as _);
    print_int($globals, total_demerits!($globals, $q));
    // print(" -> @@@@");
    print($globals, crate::strpool_str!(" -> @@").get() as _);
    // if prev_break(passive)=null then print_char("0")
    if prev_break!($globals, $globals.passive) == null {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'0'),
        );
    }
    // else print_int(serial(prev_break(passive)));
    else {
        print_int(
            $globals,
            serial!($globals, prev_break!($globals, $globals.passive)) as _,
        );
    }
    // end
    use crate::pascal::integer;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0062::print_nl;
    use crate::section_0065::print_int;
    use crate::section_0115::null;
    use crate::section_0819::hyphenated;
    use crate::section_0819::line_number;
    use crate::section_0819::total_demerits;
    use crate::section_0821::prev_break;
    use crate::section_0821::serial;
}}
