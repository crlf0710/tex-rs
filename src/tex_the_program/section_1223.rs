//! ` `

// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_1223($globals:expr, $cmd:expr, $chr_code:expr) {{
    // shorthand_def: case chr_code of
    let processed = if $cmd == shorthand_def {
        // char_def_code: print_esc("chardef");
        if $chr_code.get() == char_def_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("chardef"));
        }
        // math_char_def_code: print_esc("mathchardef");
        else if $chr_code.get() == math_char_def_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("mathchardef"));
        }
        // count_def_code: print_esc("countdef");
        else if $chr_code.get() == count_def_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("countdef"));
        }
        // dimen_def_code: print_esc("dimendef");
        else if $chr_code.get() == dimen_def_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("dimendef"));
        }
        // skip_def_code: print_esc("skipdef");
        else if $chr_code.get() == skip_def_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("skipdef"));
        }
        // mu_skip_def_code: print_esc("muskipdef");
        else if $chr_code.get() == mu_skip_def_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("muskipdef"));
        }
        // othercases print_esc("toksdef")
        else {
            print_esc($globals, crate::strpool_str!("toksdef"));
        }
        // endcases;
        use crate::section_0297::chr_code_repr;
        use crate::section_1222::char_def_code;
        use crate::section_1222::count_def_code;
        use crate::section_1222::dimen_def_code;
        use crate::section_1222::math_char_def_code;
        use crate::section_1222::mu_skip_def_code;
        use crate::section_1222::skip_def_code;
        true
    }
    // char_given: begin print_esc("char"); print_hex(chr_code);
    else if $cmd == char_given {
        print_esc($globals, crate::strpool_str!("char"));
        print_hex($globals, $chr_code.get() as _);
        // end;
        use crate::section_0067::print_hex;
        true
    }
    // math_given: begin print_esc("mathchar"); print_hex(chr_code);
    else if $cmd == math_given {
        print_esc($globals, crate::strpool_str!("mathchar"));
        print_hex($globals, $chr_code.get() as _);
        // end;
        use crate::section_0067::print_hex;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::*;
    use crate::section_0209::*;
    processed
}}
