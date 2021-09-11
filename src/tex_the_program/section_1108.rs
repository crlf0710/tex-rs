//! ` `
// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1108($globals:expr, $cmd:expr, $chr_code:expr) {{
    // remove_item: if chr_code=glue_node then print_esc("unskip")
    let processed = if $cmd == remove_item {
        if $chr_code.get() == glue_node as chr_code_repr {
            print_esc($globals, crate::strpool_str!("unskip"));
        }
        // else if chr_code=kern_node then print_esc("unkern")
        else if $chr_code.get() == kern_node as chr_code_repr {
            print_esc($globals, crate::strpool_str!("unkern"));
        }
        // else print_esc("unpenalty");
        else {
            print_esc($globals, crate::strpool_str!("unpenalty"));
        }
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
        use crate::section_0297::chr_code_repr;
        true
    }
    // un_hbox: if chr_code=copy_code then print_esc("unhcopy")
    else if $cmd == un_hbox {
        if $chr_code.get() == copy_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("unhcopy"));
        }
        // else print_esc("unhbox");
        else {
            print_esc($globals, crate::strpool_str!("unhbox"));
        }
        use crate::section_0297::chr_code_repr;
        use crate::section_1071::copy_code;
        true
    }
    // un_vbox: if chr_code=copy_code then print_esc("unvcopy")
    else if $cmd == un_vbox {
        if $chr_code.get() == copy_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("unvcopy"));
        }
        // else print_esc("unvbox");
        else {
            print_esc($globals, crate::strpool_str!("unvbox"));
        }
        use crate::section_0297::chr_code_repr;
        use crate::section_1071::copy_code;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::*;
    processed
}}
