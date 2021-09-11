//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1295($globals:expr, $cmd:expr, $chr_code:expr) {{
    // undefined_cs: print("undefined");
    let processed = if $cmd == undefined_cs {
        print($globals, crate::strpool_str!("undefined").get() as _);
        true
    }
    // call: print("macro");
    else if $cmd == call {
        print($globals, crate::strpool_str!("macro").get() as _);
        true
    }
    // long_call: print_esc("long macro");
    else if $cmd == long_call {
        print_esc($globals, crate::strpool_str!("long macro"));
        true
    }
    // outer_call: print_esc("outer macro");
    else if $cmd == outer_call {
        print_esc($globals, crate::strpool_str!("outer macro"));
        true
    }
    // long_outer_call: begin print_esc("long"); print_esc("outer macro");
    //   end;
    else if $cmd == long_outer_call {
        print_esc($globals, crate::strpool_str!("long"));
        print_esc($globals, crate::strpool_str!("outer macro"));
        true
    }
    // end_template: print_esc("outer endtemplate");
    else if $cmd == end_template {
        print_esc($globals, crate::strpool_str!("outer endtemplate"));
        true
    } else {
        false
    };
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0210::*;
    processed
}}
