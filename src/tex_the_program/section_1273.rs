//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1273($globals:expr, $cmd:expr, $chr_code:expr) {{
    // in_stream: if chr_code=0 then print_esc("closein")
    let processed = if $cmd == in_stream {
        if $chr_code.get() == 0 {
            print_esc($globals, crate::strpool_str!("closein"));
        }
        // else print_esc("openin");
        else {
            print_esc($globals, crate::strpool_str!("openin"));
        }
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::*;
    processed
}}
