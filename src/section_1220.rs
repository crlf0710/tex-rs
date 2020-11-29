//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1220 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // let: if chr_code<>normal then print_esc("futurelet")@+else print_esc("let");
        if $cmd == r#let {
            if $chr_code.get() != let_kind::normal as _ {
                print_esc($globals, strpool_str!("futurelet"));
            } else {
                print_esc($globals, strpool_str!("let"));
            }
            use crate::section_0135::let_kind;
            true
        } else {
            false
        }
    }}
}
