//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1292($globals:expr, $cmd:expr, $chr_code:expr) {{
    // xray: case chr_code of
    let processed = if $cmd == xray {
        // show_box_code:print_esc("showbox");
        if $chr_code.get() == show_box_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("showbox"));
        }
        // show_the_code:print_esc("showthe");
        else if $chr_code.get() == show_the_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("showthe"));
        }
        // show_lists_code:print_esc("showlists");
        else if $chr_code.get() == show_lists_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("showlists"));
        }
        // othercases print_esc("show")
        else {
            print_esc($globals, crate::strpool_str!("show"));
        }
        // endcases;
        use crate::section_0297::chr_code_repr;
        use crate::section_1291::show_kind::*;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::*;
    processed
}}
