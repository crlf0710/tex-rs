//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1287($globals:expr, $cmd:expr, $chr_code:expr) {{
    // case_shift:if chr_code=lc_code_base then print_esc("lowercase")
    let processed = if $cmd == case_shift {
        if $chr_code.get() == lc_code_base {
            print_esc($globals, crate::strpool_str!("lowercase"));
        }
        // else print_esc("uppercase");
        else {
            print_esc($globals, crate::strpool_str!("uppercase"));
        }
        use crate::section_0230::lc_code_base;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::*;
    processed
}}
