//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0239 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // assign_int: if chr_code<count_base then print_param(chr_code-int_base)
        if $cmd == assign_int {
            if $chr_code.get() < count_base {
                print_param($globals, $chr_code.get() as integer - int_base as integer);
            }
            // else  begin print_esc("count"); print_int(chr_code-count_base);
            else {
                print_esc($globals, strpool_str!("count"));
                print_int($globals, $chr_code.get() as integer - count_base as integer);
                // end;
            }
            true
        } else {
            false
        }
    }}
}

