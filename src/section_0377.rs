//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0377 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // input: if chr_code=0 then print_esc("input")@+else print_esc("endinput");
        if $cmd == input {
            if $chr_code.get() == 0 {
                print_esc($globals, strpool_str!("input"));
            } else {
                print_esc($globals, strpool_str!("endinput"));
            }
            true
        } else {
            false
        }
    }}
}
