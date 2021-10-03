//! ` `
// @<Display the page break cost@>=
#[cfg(feature = "statistics")]
pub(crate) macro Display_the_page_break_cost($globals:expr, $b:expr, $pi:expr, $c:expr) {{
    // begin begin_diagnostic; print_nl("%");
    begin_diagnostic($globals);
    print_nl($globals, crate::strpool_str!("%"));
    // print(" t="); print_totals;@/
    print($globals, crate::strpool_str!(" t=").get() as _);
    print_totals($globals);
    // print(" g="); print_scaled(page_goal);@/
    print($globals, crate::strpool_str!(" g=").get() as _);
    print_scaled($globals, page_goal!($globals));
    // print(" b=");
    print($globals, crate::strpool_str!(" b=").get() as _);
    // if b=awful_bad then print_char("*")@+else print_int(b);
    if $b == awful_bad {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'*'),
        );
    } else {
        print_int($globals, $b);
    }
    // @.*\relax@>
    // print(" p="); print_int(pi);
    print($globals, crate::strpool_str!(" p=").get() as _);
    print_int($globals, $pi);
    // print(" c=");
    print($globals, crate::strpool_str!(" c=").get() as _);
    // if c=awful_bad then print_char("*")@+else print_int(c);
    if $c == awful_bad {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'*'),
        );
    } else {
        print_int($globals, $c);
    }
    // if c<=least_page_cost then print_char("#");
    if $c <= $globals.least_page_cost {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'#'),
        );
    }
    // end_diagnostic(false);
    end_diagnostic($globals, false);
    // end
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0062::print_nl;
    use crate::section_0065::print_int;
    use crate::section_0103::print_scaled;
    use crate::section_0108::inf_bad;
    use crate::section_0245::begin_diagnostic;
    use crate::section_0245::end_diagnostic;
    use crate::section_0833::awful_bad;
    use crate::section_0982::page_goal;
    use crate::section_0985::print_totals;
}}
