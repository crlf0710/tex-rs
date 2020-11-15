//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0335 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // par_end:print_esc("par");
        if $cmd == par_end {
            print_esc($globals, strpool_str!("par"));
            true
        } else {
            false
        }
    }}
}
