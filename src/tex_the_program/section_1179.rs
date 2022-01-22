//! ` `
// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1179($globals:expr, $cmd:expr, $chr_code:expr) {{
    // above: case chr_code of
    let processed = if $cmd == above {
        // over_code:print_esc("over");
        if $chr_code.get() == over_code as _ {
            print_esc($globals, crate::strpool_str!("over"));
        }
        // atop_code:print_esc("atop");
        else if $chr_code.get() == atop_code as _ {
            print_esc($globals, crate::strpool_str!("atop"));
        }
        // delimited_code+above_code:print_esc("abovewithdelims");
        else if $chr_code.get() == (delimited_code + above_code) as _ {
            print_esc($globals, crate::strpool_str!("abovewithdelims"));
        }
        // delimited_code+over_code:print_esc("overwithdelims");
        else if $chr_code.get() == (delimited_code + over_code) as _ {
            print_esc($globals, crate::strpool_str!("overwithdelims"));
        }
        // delimited_code+atop_code:print_esc("atopwithdelims");
        else if $chr_code.get() == (delimited_code + atop_code) as _ {
            print_esc($globals, crate::strpool_str!("atopwithdelims"));
        }
        // othercases print_esc("above")
        else {
            print_esc($globals, crate::strpool_str!("above"));
        }
        // endcases;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::above;
    use crate::section_1178::above_code;
    use crate::section_1178::atop_code;
    use crate::section_1178::delimited_code;
    use crate::section_1178::over_code;
    processed
}}
