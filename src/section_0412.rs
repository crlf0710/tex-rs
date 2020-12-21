//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0412 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // register: if chr_code=int_val then print_esc("count")
        if $cmd == register {
            if $chr_code.get() == int_val as chr_code_repr {
                print_esc($globals, strpool_str!("count"));
            }
            // else if chr_code=dimen_val then print_esc("dimen")
            else if $chr_code.get() == dimen_val as chr_code_repr {
                print_esc($globals, strpool_str!("dimen"));
            }
            // else if chr_code=glue_val then print_esc("skip")
            else if $chr_code.get() == glue_val as chr_code_repr {
                print_esc($globals, strpool_str!("skip"));
            }
            // else print_esc("muskip");
            else {
                print_esc($globals, strpool_str!("muskip"));
            }
            use crate::section_0297::chr_code_repr;
            use crate::section_0410::cur_val_level_kind::*;
            true
        } else {
            false
        }
    }}
}