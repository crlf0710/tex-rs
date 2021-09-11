//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0239($globals:expr, $cmd:expr, $chr_code:expr) {{
    // assign_int: if chr_code<count_base then print_param(chr_code-int_base)
    let processed = if $cmd == assign_int {
        if $chr_code.get() < count_base {
            print_param($globals, $chr_code.get() as integer - int_base as integer);
        }
        // else  begin print_esc("count"); print_int(chr_code-count_base);
        else {
            print_esc($globals, crate::strpool_str!("count"));
            print_int($globals, $chr_code.get() as integer - count_base as integer);
            // end;
        }
        true
    } else {
        false
    };
    use crate::pascal::integer;
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0209::*;
    use crate::section_0230::int_base;
    use crate::section_0236::count_base;
    use crate::section_0237::print_param;
    processed
}}
