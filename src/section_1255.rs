//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1255 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // assign_font_int: if chr_code=0 then print_esc("hyphenchar")
        if $cmd == assign_font_int {
            if $chr_code.get() == 0 {
                print_esc($globals, strpool_str!("hyphenchar"));
            }
            // else print_esc("skewchar");
            else {
                print_esc($globals, strpool_str!("skewchar"));
            }
            true
        } else {
            false
        }
    }}
}
