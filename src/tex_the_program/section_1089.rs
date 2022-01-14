//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1089($globals:expr, $cmd:expr, $chr_code:expr) {{
    // start_par: if chr_code=0 then print_esc("noindent")@+ else print_esc("indent");
    let processed = if $cmd == start_par {
        if $chr_code.get() == 0 {
            print_esc($globals, crate::strpool_str!("noindent"));
        } else {
            print_esc($globals, crate::strpool_str!("indent"));
        }
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::start_par;
    processed
}}
