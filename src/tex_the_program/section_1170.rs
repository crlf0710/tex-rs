//! ` `
// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1170($globals:expr, $cmd:expr, $chr_code:expr) {{
    // math_style: print_style(chr_code);
    let processed = if $cmd == math_style {
        print_style($globals, $chr_code.get() as _);
        true
    } else {
        false
    };
    use crate::section_0208::math_style;
    use crate::section_0694::print_style;
    processed
}}
