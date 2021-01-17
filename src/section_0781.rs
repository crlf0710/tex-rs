//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0781 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // tab_mark: if chr_code=span_code then print_esc("span")
        if $cmd == tab_mark {
            if $chr_code.get() as integer == span_code as integer {
                print_esc($globals, strpool_str!("span"));
            }
            // else chr_cmd("alignment tab character ");
            else {
                chr_cmd!($globals, $chr_code, strpool_str!("alignment tab character "));
            }
            use crate::section_0780::span_code;
            true
        }
        // car_ret: if chr_code=cr_code then print_esc("cr")
        else if $cmd == car_ret {
            if $chr_code.get() as integer == cr_code as integer {
                print_esc($globals, strpool_str!("cr"));
            }
            // else print_esc("crcr");
            else {
                print_esc($globals, strpool_str!("crcr"));
            }
            use crate::section_0780::cr_code;
            true
        } else {
            false
        }
    }}
}
