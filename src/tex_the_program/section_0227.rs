//! ` `

// @<Cases of |print_cmd_chr| for symbolic printing of primitives@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0227($globals:expr, $cmd:expr, $chr_code:expr) {{
    // assign_glue,assign_mu_glue: if chr_code<skip_base then
    let processed = if $cmd == assign_glue || $cmd == assign_mu_glue {
        if $chr_code.get() < skip_base {
            // print_skip_param(chr_code-glue_base)
            print_skip_param($globals, ($chr_code.get() - glue_base) as integer);
        }
        // else if chr_code<mu_skip_base then
        else if $chr_code.get() < mu_skip_base {
            // begin print_esc("skip"); print_int(chr_code-skip_base);
            print_esc($globals, crate::strpool_str!("skip"));
            print_int($globals, ($chr_code.get() - skip_base) as integer);
            // end
        }
        // else  begin print_esc("muskip"); print_int(chr_code-mu_skip_base);
        else {
            print_esc($globals, crate::strpool_str!("muskip"));
            print_int($globals, ($chr_code.get() - mu_skip_base) as integer);
            // end;
        }
        true
    } else {
        false
    };
    use crate::pascal::integer;
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0209::*;
    use crate::section_0222::glue_base;
    use crate::section_0224::mu_skip_base;
    use crate::section_0224::skip_base;
    use crate::section_0225::print_skip_param;
    processed
}}
