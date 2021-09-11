//! ` `
// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1251($globals:expr, $cmd:expr, $chr_code:expr) {{
    // hyph_data: if chr_code=1 then print_esc("patterns")
    let processed = if $cmd == hyph_data {
        if $chr_code.get() == 1 {
            print_esc($globals, crate::strpool_str!("patterns"));
        }
        // else print_esc("hyphenation");
        else {
            print_esc($globals, crate::strpool_str!("hyphenation"));
        }
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0209::*;
    processed
}}
