//! ` `
// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1278 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // message: if chr_code=0 then print_esc("message")
        if $cmd == message {
            if $chr_code.get() == 0 {
                print_esc($globals, strpool_str!("message"));
            }
            // else print_esc("errmessage");
            else {
                print_esc($globals, strpool_str!("errmessage"));
            }
            true
        } else {
            false
        }
    }}
}

