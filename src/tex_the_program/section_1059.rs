//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1059 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // hskip: case chr_code of
        if $cmd == hskip {
            let chr_code = $chr_code.get();
            // skip_code:print_esc("hskip");
            if chr_code == skip_code as chr_code_repr {
                print_esc($globals, strpool_str!("hskip"));
            }
            // fil_code:print_esc("hfil");
            else if chr_code == fil_code as chr_code_repr {
                print_esc($globals, strpool_str!("hfil"));
            }
            // fill_code:print_esc("hfill");
            else if chr_code == fill_code as chr_code_repr {
                print_esc($globals, strpool_str!("hfill"));
            }
            // ss_code:print_esc("hss");
            else if chr_code == ss_code as chr_code_repr {
                print_esc($globals, strpool_str!("hss"));
            }
            // othercases print_esc("hfilneg")
            else {
                print_esc($globals, strpool_str!("hfilneg"));
            }
            // endcases;
            use crate::section_0297::chr_code_repr;
            use crate::section_1058::*;
            true
        }
        // vskip: case chr_code of
        else if $cmd == vskip {
            let chr_code = $chr_code.get();
            // skip_code:print_esc("vskip");
            if chr_code == skip_code as chr_code_repr {
                print_esc($globals, strpool_str!("vskip"));
            }
            // fil_code:print_esc("vfil");
            else if chr_code == fil_code as chr_code_repr {
                print_esc($globals, strpool_str!("vfil"));
            }
            // fill_code:print_esc("vfill");
            else if chr_code == fill_code as chr_code_repr {
                print_esc($globals, strpool_str!("vfill"));
            }
            // ss_code:print_esc("vss");
            else if chr_code == ss_code as chr_code_repr {
                print_esc($globals, strpool_str!("vss"));
            }
            // othercases print_esc("vfilneg")
            else {
                print_esc($globals, strpool_str!("vfilneg"));
            }
            // endcases;
            use crate::section_0297::chr_code_repr;
            use crate::section_1058::*;
            true
        }
        // mskip: print_esc("mskip");
        else if $cmd == mskip {
            print_esc($globals, strpool_str!("mskip"));
            true
        }
        // kern: print_esc("kern");
        else if $cmd == kern {
            print_esc($globals, strpool_str!("kern"));
            true
        }
        // mkern: print_esc("mkern");
        else if $cmd == mkern {
            print_esc($globals, strpool_str!("mkern"));
            true
        } else {
            false
        }
    }}
}

