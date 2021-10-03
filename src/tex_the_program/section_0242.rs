//! ` `

// @<Show equivalent |n|, in region 5@>=
#[cfg(feature = "statistics")]
pub(crate) macro Show_equivalent_n__in_region_5($globals:expr, $n:expr) {{
    // begin if n<count_base then print_param(n-int_base)
    if ($n as integer) < count_base as integer {
        print_param($globals, $n as integer - int_base as integer);
    }
    // else if  n<del_code_base then
    else if ($n as integer) < del_code_base as integer {
        // begin print_esc("count"); print_int(n-count_base);
        print_esc($globals, crate::strpool_str!("count"));
        print_int($globals, $n as integer - count_base as integer);
        // end
    }
    // else  begin print_esc("delcode"); print_int(n-del_code_base);
    else {
        print_esc($globals, crate::strpool_str!("delcode"));
        print_int($globals, $n as integer - del_code_base as integer);
        // end;
    }
    // print_char("="); print_int(eqtb[n].int);
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b'='),
    );
    print_int($globals, $globals.eqtb[$n][MEMORY_WORD_INT]);
    // end
    use crate::pascal::integer;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0113::MEMORY_WORD_INT;
    use crate::section_0230::int_base;
    use crate::section_0236::count_base;
    use crate::section_0236::del_code_base;
    use crate::section_0237::print_param;
}}
