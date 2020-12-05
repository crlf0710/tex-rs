//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1292 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // xray: case chr_code of
        if $cmd == xray {
            // show_box_code:print_esc("showbox");
            if $chr_code.get() == show_box_code as _ {
                print_esc($globals, strpool_str!("showbox"));
            }
            // show_the_code:print_esc("showthe");
            else if $chr_code.get() == show_the_code as _ {
                print_esc($globals, strpool_str!("showthe"));
            }
            // show_lists:print_esc("showlists");
            else if $chr_code.get() == show_lists as _ {
                print_esc($globals, strpool_str!("showlists"));
            }
            // othercases print_esc("show")
            else {
                print_esc($globals, strpool_str!("show"));
            }
            // endcases;
            use crate::section_1291::show_kind::*;
            true
        } else {
            false
        }
    }}
}
