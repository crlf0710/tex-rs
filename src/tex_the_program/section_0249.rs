//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0249($globals:expr, $cmd:expr, $chr_code:expr) {{
    // assign_dimen: if chr_code<scaled_base then
    let processed = if $cmd == assign_dimen {
        if $chr_code.get() < scaled_base {
            // print_length_param(chr_code-dimen_base)
            print_length_param($globals, ($chr_code.get() - dimen_base) as _);
        }
        // else  begin print_esc("dimen"); print_int(chr_code-scaled_base);
        else {
            print_esc($globals, crate::strpool_str!("dimen"));
            print_int($globals, ($chr_code.get() - scaled_base) as integer);
        }
        // end;
        use crate::pascal::integer;
        use crate::section_0236::dimen_base;
        use crate::section_0247::print_length_param;
        use crate::section_0247::scaled_base;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0209::*;
    processed
}}
