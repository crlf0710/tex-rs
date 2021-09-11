//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0492($globals:expr, $cmd:expr, $chr_code:expr) {{
    // fi_or_else: if chr_code=fi_code then print_esc("fi")
    let processed = if $cmd == fi_or_else {
        if $chr_code.get() == fi_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("fi"));
        }
        // else if chr_code=or_code then print_esc("or")
        else if $chr_code.get() == or_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("or"));
        }
        // else print_esc("else");
        else {
            print_esc($globals, crate::strpool_str!("else"));
        }
        use crate::section_0297::chr_code_repr;
        use crate::section_0489::*;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0210::*;
    processed
}}
