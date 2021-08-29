//! ` `
// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0984 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        // set_page_dimen: case chr_code of
        if $cmd == set_page_dimen {
            // 0: print_esc("pagegoal");
            if $chr_code.get() == 0 {
                print_esc($globals, strpool_str!("pagegoal"));
            }
            // 1: print_esc("pagetotal");
            else if $chr_code.get() == 1 {
                print_esc($globals, strpool_str!("pagetotal"));
            }
            // 2: print_esc("pagestretch");
            else if $chr_code.get() == 2 {
                print_esc($globals, strpool_str!("pagestretch"));
            }
            // 3: print_esc("pagefilstretch");
            else if $chr_code.get() == 3 {
                print_esc($globals, strpool_str!("pagefilstretch"));
            }
            // 4: print_esc("pagefillstretch");
            else if $chr_code.get() == 4 {
                print_esc($globals, strpool_str!("pagefillstretch"));
            }
            // 5: print_esc("pagefilllstretch");
            else if $chr_code.get() == 5 {
                print_esc($globals, strpool_str!("pagefilllstretch"));
            }
            // 6: print_esc("pageshrink");
            else if $chr_code.get() == 6 {
                print_esc($globals, strpool_str!("pageshrink"));
            }
            // othercases print_esc("pagedepth")
            else {
                print_esc($globals, strpool_str!("pagedepth"));
            }
            // endcases;
            true
        } else {
            false
        }
    }}
}
