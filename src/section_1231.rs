//! ` `

// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1231 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        if $cmd == def_code {
            // def_code: if chr_code=cat_code_base then print_esc("catcode")
            if $chr_code.get() == cat_code_base {
                print_esc($globals, strpool_str!("catcode"));
            }
            // else if chr_code=math_code_base then print_esc("mathcode")
            else if $chr_code.get() == math_code_base {
                print_esc($globals, strpool_str!("mathcode"));
            }
            // else if chr_code=lc_code_base then print_esc("lccode")
            else if $chr_code.get() == lc_code_base {
                print_esc($globals, strpool_str!("lccode"));
            }
            // else if chr_code=uc_code_base then print_esc("uccode")
            else if $chr_code.get() == uc_code_base {
                print_esc($globals, strpool_str!("uccode"));
            }
            // else if chr_code=sf_code_base then print_esc("sfcode")
            else if $chr_code.get() == sf_code_base {
                print_esc($globals, strpool_str!("sfcode"));
            }
            // else print_esc("delcode");
            else {
                print_esc($globals, strpool_str!("delcode"));
            }

            use crate::section_0230::*;

            true
        } else if $cmd == def_family {
            // def_family: print_size(chr_code-math_font_base);
            todo!("def_family");
            true
        } else {
            false
        }
    }}
}
