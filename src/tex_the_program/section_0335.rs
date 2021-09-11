//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0335($globals:expr, $cmd:expr, $chr_code:expr) {{
    // par_end:print_esc("par");
    let processed = if $cmd == par_end {
        print_esc($globals, crate::strpool_str!("par"));
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0207::*;
    processed
}}
