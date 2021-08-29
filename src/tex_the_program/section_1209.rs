//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1209 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        if $cmd == prefix {
            // prefix: if chr_code=1 then print_esc("long")
            if $chr_code.get() == 1 {
                print_esc($globals, strpool_str!("long"));
            }
            // else if chr_code=2 then print_esc("outer")
            else if $chr_code.get() == 2 {
                print_esc($globals, strpool_str!("outer"));
            }
            // else print_esc("global");
            else {
                print_esc($globals, strpool_str!("global"));
            }
            true
        }
        // def: if chr_code=0 then print_esc("def")
        else if $cmd == def {
            if $chr_code.get() == 0 {
                print_esc($globals, strpool_str!("def"));
            }
            // else if chr_code=1 then print_esc("gdef")
            else if $chr_code.get() == 1 {
                print_esc($globals, strpool_str!("gdef"));
            }
            // else if chr_code=2 then print_esc("edef")
            else if $chr_code.get() == 2 {
                print_esc($globals, strpool_str!("edef"));
            }
            // else print_esc("xdef");
            else {
                print_esc($globals, strpool_str!("xdef"));
            }
            true
        } else {
            false
        }
    }}
}
