//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1295 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // undefined_cs: print("undefined");
        if $cmd == undefined_cs {
            print($globals, strpool_str!("undefined").get() as _);
            true
        }
        // call: print("macro");
        else if $cmd == call {
            print($globals, strpool_str!("macro").get() as _);
            true
        }
        // long_call: print_esc("long macro");
        else if $cmd == long_call {
            print_esc($globals, strpool_str!("long macro"));
            true
        }
        // outer_call: print_esc("outer macro");
        else if $cmd == outer_call {
            print_esc($globals, strpool_str!("outer macro"));
            true
        }
        // long_outer_call: begin print_esc("long"); print_esc("outer macro");
        //   end;
        else if $cmd == long_outer_call {
            print_esc($globals, strpool_str!("long"));
            print_esc($globals, strpool_str!("outer macro"));
            true
        }
        // end_template: print_esc("outer endtemplate");
        else if $cmd == end_template {
            print_esc($globals, strpool_str!("outer endtemplate"));
            true
        }
        else {
            false
        }
    }}
}
