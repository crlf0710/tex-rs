//! @ Here is a routine that displays the current meaning of an |eqtb| entry
//! in region 1 or~2. (Similar routines for the other regions will appear
//! below.)
//
// @<Show equivalent |n|, in region 1 or 2@>=
pub(crate) macro Show_equivalent_n__in_region_1_or_2($globals:expr, $n:expr) {{
    // begin sprint_cs(n); print_char("="); print_cmd_chr(eq_type(n),equiv(n));
    sprint_cs($globals, $n);
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b'='),
    );
    print_cmd_chr(
        $globals,
        eq_type!($globals, $n),
        chr_code_type::new(equiv!($globals, $n) as _),
    );
    // if eq_type(n)>=call then
    if eq_type!($globals, $n) >= call {
        // begin print_char(":"); show_token_list(link(equiv(n)),null,32);
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b':'),
        );
        show_token_list(
            $globals,
            link!($globals, equiv!($globals, $n)) as _,
            null as _,
            32,
        );
        // end;
    }
    // end
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0210::call;
    use crate::section_0221::eq_type;
    use crate::section_0221::equiv;
    use crate::section_0263::sprint_cs;
    use crate::section_0292::show_token_list;
    use crate::section_0297::chr_code_type;
    use crate::section_0298::print_cmd_chr;
}}
