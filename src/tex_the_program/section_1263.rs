//! ` `
// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1263($globals:expr, $cmd:expr, $chr_code:expr) {{
    // set_interaction: case chr_code of
    let processed = if $cmd == set_interaction {
        // batch_mode: print_esc("batchmode");
        if $chr_code.get() == batch_mode as chr_code_repr {
            print_esc($globals, crate::strpool_str!("batchmode"));
        }
        // nonstop_mode: print_esc("nonstopmode");
        else if $chr_code.get() == nonstop_mode as chr_code_repr {
            print_esc($globals, crate::strpool_str!("nonstopmode"));
        }
        // scroll_mode: print_esc("scrollmode");
        else if $chr_code.get() == scroll_mode as chr_code_repr {
            print_esc($globals, crate::strpool_str!("scrollmode"));
        }
        // othercases print_esc("errorstopmode")
        else {
            print_esc($globals, crate::strpool_str!("errorstopmode"));
        }
        // endcases;
        use crate::section_0073::batch_mode;
        use crate::section_0073::nonstop_mode;
        use crate::section_0073::scroll_mode;
        use crate::section_0297::chr_code_repr;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0209::*;
    processed
}}
