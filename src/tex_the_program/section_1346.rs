//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1346($globals:expr, $cmd:expr, $chr_code:expr) {{
    // extension: case chr_code of
    let processed = if $cmd == extension {
        if false {
            unreachable!();
        }
        // open_node:print_esc("openout");
        // write_node:print_esc("write");
        else if $chr_code.get() == write_node as chr_code_repr {
            print_esc($globals, crate::strpool_str!("write"));
        }
        // close_node:print_esc("closeout");
        // special_node:print_esc("special");
        // immediate_code:print_esc("immediate");
        else if $chr_code.get() == immediate_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("immediate"));
        }
        // set_language_code:print_esc("setlanguage");
        else if $chr_code.get() == set_language_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("setlanguage"));
        }
        // othercases print("[unknown extension!]")
        else {
            print(
                $globals,
                crate::strpool_str!("[unknown extension!]").get() as _,
            );
        }
        // endcases;
        use crate::section_0059::print;
        use crate::section_0063::print_esc;
        use crate::section_0297::chr_code_repr;
        use crate::section_1341::*;
        use crate::section_1344::*;
        true
    } else {
        false
    };
    use crate::section_0208::*;
    processed
}}
