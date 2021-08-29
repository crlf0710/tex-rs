//! @ An ``explicit'' kern value is indicated implicitly by an explicit space.
//
// @<Display kern |p|@>=
macro_rules! Display_kern_p {
    ($globals:expr, $p:expr) => {{
        // if subtype(p)<>mu_glue then
        if subtype!($globals, $p as pointer) as integer != kern_node_subtype::mu_glue as integer {
            // begin print_esc("kern");
            print_esc($globals, strpool_str!("kern"));
            // if subtype(p)<>normal then print_char(" ");
            if subtype!($globals, $p as pointer) as integer != kern_node_subtype::normal as integer
            {
                print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b' '));
            }
            // print_scaled(width(p));
            print_scaled($globals, width!($globals, $p as pointer));
            // if subtype(p)=acc_kern then print(" (for accent)");
            if subtype!($globals, $p as pointer) as integer
                == kern_node_subtype::acc_kern as integer
            {
                print($globals, strpool_str!(" (for accent)").get() as _);
            }
        // @.for accent@>
        // end
        }
        // else  begin print_esc("mkern"); print_scaled(width(p)); print("mu");
        else {
            print_esc($globals, strpool_str!("mkern"));
            print_scaled($globals, width!($globals, $p as pointer));
            print($globals, strpool_str!("mu").get() as _);
            // end
        }
        use crate::section_0058::print_char;
        use crate::section_0063::print_esc;
        use crate::section_0103::print_scaled;
        use crate::section_0155::kern_node_subtype;
    }};
}
