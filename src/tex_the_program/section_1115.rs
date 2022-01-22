//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1115($globals:expr, $cmd:expr, $chr_code:expr) {{
    // discretionary: if chr_code=1 then
    //   print_esc("-")@+else print_esc("discretionary");
    let processed = if $cmd == discretionary {
        if $chr_code.get() == 1 {
            print_esc($globals, crate::strpool_str!("-"));
        } else {
            print_esc($globals, crate::strpool_str!("discretionary"));
        }
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::discretionary;
    processed
}}
