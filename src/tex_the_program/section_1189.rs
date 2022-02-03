//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1189($globals:expr, $cmd:expr, $chr_code:expr) {{
    // left_right: if chr_code=left_noad then print_esc("left")
    let processed = if $cmd == left_right {
        if $chr_code.get() == left_noad as _ {
            print_esc($globals, crate::strpool_str!("left"));
        }
        // else print_esc("right");
        else {
            print_esc($globals, crate::strpool_str!("right"));
        }
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::left_right;
    use crate::section_0687::left_noad;
    processed
}}
