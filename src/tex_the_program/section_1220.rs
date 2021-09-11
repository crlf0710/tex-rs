//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1220($globals:expr, $cmd:expr, $chr_code:expr) {{
    // let: if chr_code<>normal then print_esc("futurelet")@+else print_esc("let");
    let processed = if $cmd == r#let {
        if $chr_code.get() != let_kind::normal as chr_code_repr {
            print_esc($globals, crate::strpool_str!("futurelet"));
        } else {
            print_esc($globals, crate::strpool_str!("let"));
        }
        use crate::section_0135::let_kind;
        use crate::section_0297::chr_code_repr;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0209::*;
    processed
}}
