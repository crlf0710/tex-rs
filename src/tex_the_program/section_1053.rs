//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1053 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // stop:if chr_code=1 then print_esc("dump")@+else print_esc("end");
        if $cmd == stop {
            if $chr_code.get() == 1 {
                print_esc($globals, strpool_str!("dump"));
            } else {
                print_esc($globals, strpool_str!("end"));
            }
            true
        } else {
            false
        }
    }}
}
