//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1072($globals:expr, $cmd:expr, $chr_code:expr) {{
    // hmove: if chr_code=1 then print_esc("moveleft")@+else print_esc("moveright");
    let processed = if $cmd == hmove {
        if $chr_code.get() == 1 {
            print_esc($globals, crate::strpool_str!("moveleft"));
        } else {
            print_esc($globals, crate::strpool_str!("moveright"));
        }
        true
    }
    // vmove: if chr_code=1 then print_esc("raise")@+else print_esc("lower");
    else if $cmd == vmove {
        if $chr_code.get() == 1 {
            print_esc($globals, crate::strpool_str!("raise"));
        } else {
            print_esc($globals, crate::strpool_str!("lower"));
        }
        true
    }
    // make_box: case chr_code of
    else if $cmd == make_box {
        // box_code: print_esc("box");
        if $chr_code.get() as integer == box_code as integer {
            print_esc($globals, crate::strpool_str!("box"));
        }
        // copy_code: print_esc("copy");
        else if $chr_code.get() as integer == copy_code as integer {
            print_esc($globals, crate::strpool_str!("copy"));
        }
        // last_box_code: print_esc("lastbox");
        else if $chr_code.get() as integer == last_box_code as integer {
            print_esc($globals, crate::strpool_str!("lastbox"));
        }
        // vsplit_code: print_esc("vsplit");
        else if $chr_code.get() as integer == vsplit_code as integer {
            print_esc($globals, crate::strpool_str!("vsplit"));
        }
        // vtop_code: print_esc("vtop");
        else if $chr_code.get() as integer == vtop_code as integer {
            print_esc($globals, crate::strpool_str!("vtop"));
        }
        // vtop_code+vmode: print_esc("vbox");
        else if $chr_code.get() as integer == vtop_code as integer + vmode as integer {
            print_esc($globals, crate::strpool_str!("vbox"));
        }
        // othercases print_esc("hbox")
        else {
            print_esc($globals, crate::strpool_str!("hbox"));
        }
        // endcases;
        use crate::section_0211::vmode;
        use crate::section_1071::box_code;
        use crate::section_1071::copy_code;
        use crate::section_1071::last_box_code;
        use crate::section_1071::vsplit_code;
        use crate::section_1071::vtop_code;
        true
    }
    // leader_ship: if chr_code=a_leaders then print_esc("leaders")
    else if $cmd == leader_ship {
        if $chr_code.get() as integer == glue_node_subtype::a_leaders as integer {
            print_esc($globals, crate::strpool_str!("leaders"));
        }
        // else if chr_code=c_leaders then print_esc("cleaders")
        else if $chr_code.get() as integer == glue_node_subtype::c_leaders as integer {
            print_esc($globals, crate::strpool_str!("cleaders"));
        }
        // else if chr_code=x_leaders then print_esc("xleaders")
        else if $chr_code.get() as integer == glue_node_subtype::x_leaders as integer {
            print_esc($globals, crate::strpool_str!("xleaders"));
        }
        // else print_esc("shipout");
        else {
            print_esc($globals, crate::strpool_str!("shipout"));
        }
        use crate::section_0149::glue_node_subtype;
        true
    } else {
        false
    };
    use crate::pascal::integer;
    use crate::section_0063::print_esc;
    use crate::section_0208::*;
    processed
}}
