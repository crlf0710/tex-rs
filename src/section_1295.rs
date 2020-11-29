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
        // long_call: print_esc("long macro");
        // outer_call: print_esc("outer macro");
        // long_outer_call: begin print_esc("long"); print_esc("outer macro");
        //   end;
        // end_template: print_esc("outer endtemplate");
        else {
            false
        }
    }}
}
