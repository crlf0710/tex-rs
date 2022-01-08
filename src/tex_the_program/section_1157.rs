//! ` `
// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1157($globals:expr, $cmd:expr, $chr_code:expr) {{
    // math_comp: case chr_code of
    let processed = if $cmd == math_comp {
        // ord_noad: print_esc("mathord");
        if $chr_code.get() == ord_noad as _ {
            print_esc($globals, crate::strpool_str!("mathord"));
        }
        // op_noad: print_esc("mathop");
        else if $chr_code.get() == op_noad as _ {
            print_esc($globals, crate::strpool_str!("mathop"));
        }
        // bin_noad: print_esc("mathbin");
        else if $chr_code.get() == bin_noad as _ {
            print_esc($globals, crate::strpool_str!("mathbin"));
        }
        // rel_noad: print_esc("mathrel");
        else if $chr_code.get() == rel_noad as _ {
            print_esc($globals, crate::strpool_str!("mathrel"));
        }
        // open_noad: print_esc("mathopen");
        else if $chr_code.get() == open_noad as _ {
            print_esc($globals, crate::strpool_str!("mathopen"));
        }
        // close_noad: print_esc("mathclose");
        else if $chr_code.get() == close_noad as _ {
            print_esc($globals, crate::strpool_str!("mathclose"));
        }
        // punct_noad: print_esc("mathpunct");
        else if $chr_code.get() == punct_noad as _ {
            print_esc($globals, crate::strpool_str!("mathpunct"));
        }
        // inner_noad: print_esc("mathinner");
        else if $chr_code.get() == inner_noad as _ {
            print_esc($globals, crate::strpool_str!("mathinner"));
        }
        // under_noad: print_esc("underline");
        else if $chr_code.get() == under_noad as _ {
            print_esc($globals, crate::strpool_str!("underline"));
        }
        // othercases print_esc("overline")
        else {
            print_esc($globals, crate::strpool_str!("overline"));
        }
        // endcases;
        use crate::section_0682::*;
        use crate::section_0687::*;
        true
    }
    // limit_switch: if chr_code=limits then print_esc("limits")
    else if $cmd == limit_switch {
        // else if chr_code=no_limits then print_esc("nolimits")
        // else print_esc("displaylimits");
        todo!("limit_switch");
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::*;
    processed
}}
