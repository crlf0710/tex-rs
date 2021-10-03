//! ` `
// @<Show equivalent |n|, in region 6@>=
#[cfg(feature = "statistics")]
pub(crate) macro Show_equivalent_n__in_region_6($globals:expr, $n:expr) {{
    // begin if n<scaled_base then print_length_param(n-dimen_base)
    if ($n as integer) < scaled_base as integer {
        print_length_param($globals, $n as integer - dimen_base as integer);
    }
    // else  begin print_esc("dimen"); print_int(n-scaled_base);
    else {
        print_esc($globals, crate::strpool_str!("dimen"));
        print_int($globals, $n as integer - scaled_base as integer);
        // end;
    }
    // print_char("="); print_scaled(eqtb[n].sc); print("pt");
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b'='),
    );
    print_scaled($globals, $globals.eqtb[$n][MEMORY_WORD_SC]);
    print($globals, crate::strpool_str!("pt").get() as _);
    // end
    use crate::pascal::integer;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0101::MEMORY_WORD_SC;
    use crate::section_0103::print_scaled;
    use crate::section_0236::dimen_base;
    use crate::section_0247::print_length_param;
    use crate::section_0247::scaled_base;
}}
