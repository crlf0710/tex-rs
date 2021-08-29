//! ` `

// @<Cases of |print_cmd_chr| for symbolic printing of primitives@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0227 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // assign_glue,assign_mu_glue: if chr_code<skip_base then
        if $cmd == assign_glue || $cmd == assign_mu_glue {
            if $chr_code.get() < skip_base {
                // print_skip_param(chr_code-glue_base)
                print_skip_param($globals, ($chr_code.get() - glue_base) as integer);
            }
            // else if chr_code<mu_skip_base then
            else if $chr_code.get() < mu_skip_base {
                // begin print_esc("skip"); print_int(chr_code-skip_base);
                print_esc($globals, strpool_str!("skip"));
                print_int($globals, ($chr_code.get() - skip_base) as integer);
                // end
            }
            // else  begin print_esc("muskip"); print_int(chr_code-mu_skip_base);
            else {
                todo!("muskip");
                // end;
            }
            use crate::section_0222::glue_base;
            use crate::section_0224::skip_base;
            use crate::section_0224::mu_skip_base;
            use crate::section_0225::print_skip_param;
            true
        } else {
            false
        }
    }}
}
