//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1053($globals:expr, $cmd:expr, $chr_code:expr) {{
    // stop:if chr_code=1 then print_esc("dump")@+else print_esc("end");
    let processed = if $cmd == stop {
        if $chr_code.get() == 1 {
            print_esc($globals, crate::strpool_str!("dump"));
        } else {
            print_esc($globals, crate::strpool_str!("end"));
        }
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0207::*;
    processed
}}
