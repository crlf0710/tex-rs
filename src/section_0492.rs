//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0492 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // fi_or_else: if chr_code=fi_code then print_esc("fi")
        if $cmd == fi_or_else {
            if $chr_code.get() == fi_code as chr_code_repr {
                print_esc($globals, strpool_str!("fi"));
            }
            // else if chr_code=or_code then print_esc("or")
            else if $chr_code.get() == or_code as chr_code_repr {
                print_esc($globals, strpool_str!("or"));
            }
            // else print_esc("else");
            else {
                print_esc($globals, strpool_str!("else"));
            }
            use crate::section_0297::chr_code_repr;
            use crate::section_0489::*;
            true
        } else {
            false
        }
    }}
}