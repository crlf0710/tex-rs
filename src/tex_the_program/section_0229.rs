//! ` `
// @<Show equivalent |n|, in region 3@>=
#[cfg(feature = "statistics")]
pub(crate) macro Show_equivalent_n__in_region_3($globals:expr, $n:expr) {{
    // if n<skip_base then
    if ($n as integer) < skip_base as integer {
        // begin print_skip_param(n-glue_base); print_char("=");
        print_skip_param($globals, $n as integer - glue_base as integer);
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'='),
        );
        // if n<glue_base+thin_mu_skip_code then print_spec(equiv(n),"pt")
        if ($n as integer) < glue_base as integer + thin_mu_skip_code as integer {
            print_spec(
                $globals,
                equiv!($globals, $n) as _,
                crate::strpool_str!("pt"),
            );
        }
        // else print_spec(equiv(n),"mu");
        else {
            print_spec(
                $globals,
                equiv!($globals, $n) as _,
                crate::strpool_str!("mu"),
            );
        }
        // end
    }
    // else if n<mu_skip_base then
    else if ($n as integer) < mu_skip_base as integer {
        // begin print_esc("skip"); print_int(n-skip_base); print_char("=");
        print_esc($globals, crate::strpool_str!("skip"));
        print_int($globals, $n as integer - skip_base as integer);
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'='),
        );
        // print_spec(equiv(n),"pt");
        print_spec(
            $globals,
            equiv!($globals, $n) as _,
            crate::strpool_str!("pt"),
        );
        // end
    }
    // else  begin print_esc("muskip"); print_int(n-mu_skip_base); print_char("=");
    else {
        // print_spec(equiv(n),"mu");
        // end
        todo!("muskip");
    }
    use crate::pascal::integer;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0178::print_spec;
    use crate::section_0221::equiv;
    use crate::section_0222::glue_base;
    use crate::section_0224::mu_skip_base;
    use crate::section_0224::skip_base;
    use crate::section_0224::thin_mu_skip_code;
    use crate::section_0225::print_skip_param;
}}
