//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1072 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // hmove: if chr_code=1 then print_esc("moveleft")@+else print_esc("moveright");
        if $cmd == hmove {
            todo!("hmove");
            true
        }
        // vmove: if chr_code=1 then print_esc("raise")@+else print_esc("lower");
        else if $cmd == vmove {
            todo!("vmove");
            true
        }
        // make_box: case chr_code of
        else if $cmd == make_box {
            // box_code: print_esc("box");
            // copy_code: print_esc("copy");
            // last_box_code: print_esc("lastbox");
            // vsplit_code: print_esc("vsplit");
            // vtop_code: print_esc("vtop");
            // vtop_code+vmode: print_esc("vbox");
            // othercases print_esc("hbox")
            // endcases;
            todo!("make_box");
            true
        }
        // leader_ship: if chr_code=a_leaders then print_esc("leaders")
        else if $cmd == leader_ship {
            // else if chr_code=c_leaders then print_esc("cleaders")
            // else if chr_code=x_leaders then print_esc("xleaders")
            // else print_esc("shipout");
            todo!("leader_ship");
            true
        } else {
            false
        }
    }}
}