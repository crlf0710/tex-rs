//! ` `
// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1278($globals:expr, $cmd:expr, $chr_code:expr) {{
    // message: if chr_code=0 then print_esc("message")
    let processed = if $cmd == message {
        if $chr_code.get() == 0 {
            print_esc($globals, crate::strpool_str!("message"));
        }
        // else print_esc("errmessage");
        else {
            print_esc($globals, crate::strpool_str!("errmessage"));
        }
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::*;
    processed
}}
