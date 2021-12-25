//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0488($globals:expr, $cmd:expr, $chr_code:expr) {{
    // if_test: case chr_code of
    let processed = if $cmd == if_test {
        let chr_code = $chr_code.get();
        if false {
            unreachable!();
        }
        // if_cat_code:print_esc("ifcat");
        else if chr_code == code::if_cat_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("ifcat"));
        }
        // if_int_code:print_esc("ifnum");
        else if chr_code == code::if_int_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("ifnum"));
        }
        // if_dim_code:print_esc("ifdim");
        else if chr_code == code::if_dim_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("ifdim"));
        }
        // if_odd_code:print_esc("ifodd");
        // if_vmode_code:print_esc("ifvmode");
        // if_hmode_code:print_esc("ifhmode");
        // if_mmode_code:print_esc("ifmmode");
        // if_inner_code:print_esc("ifinner");
        // if_void_code:print_esc("ifvoid");
        else if chr_code == code::if_void_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("ifvoid"));
        }
        // if_hbox_code:print_esc("ifhbox");
        // if_vbox_code:print_esc("ifvbox");
        // ifx_code:print_esc("ifx");
        else if chr_code == code::ifx_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("ifx"));
        }
        // if_eof_code:print_esc("ifeof");
        else if chr_code == code::if_eof_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("ifeof"));
        }
        // if_true_code:print_esc("iftrue");
        // if_false_code:print_esc("iffalse");
        else if chr_code == code::if_false_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("iffalse"));
        }
        // if_case_code:print_esc("ifcase");
        // othercases print_esc("if")
        else {
            print_esc($globals, crate::strpool_str!("if"));
        }
        // endcases;
        use crate::section_0297::chr_code_repr;
        use crate::section_0487 as code;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0210::*;
    processed
}}
