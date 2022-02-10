//! ` `
// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1143($globals:expr, $cmd:expr, $chr_code:expr) {{
    // eq_no:if chr_code=1 then print_esc("leqno")@+else print_esc("eqno");
    let processed = if $cmd == eq_no {
        if $chr_code.get() == 1 {
            print_esc($globals, crate::strpool_str!("leqno"));
        } else {
            print_esc($globals, crate::strpool_str!("eqno"));
        }
        true
    } else {
        false
    };
    processed
}}
