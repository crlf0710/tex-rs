//! ` `
// @<Cases of |print_cmd_chr|...@>=
pub(crate) macro Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0417($globals:expr, $cmd:expr, $chr_code:expr) {{
    let processed = if false {
        unreachable!();
    }
    // set_aux: if chr_code=vmode then print_esc("prevdepth")
    else if $cmd == set_aux {
        if $chr_code.get() == vmode as chr_code_repr {
            print_esc($globals, crate::strpool_str!("prevdepth"));
        }
        // @+else print_esc("spacefactor");
        else {
            print_esc($globals, crate::strpool_str!("spacefactor"));
        }
        use crate::section_0211::vmode;
        use crate::section_0297::chr_code_repr;
        true
    }
    // set_page_int: if chr_code=0 then print_esc("deadcycles")
    // @+else print_esc("insertpenalties");
    // set_box_dimen: if chr_code=width_offset then print_esc("wd")
    else if $cmd == set_box_dimen {
        if $chr_code.get() == width_offset as chr_code_repr {
            print_esc($globals, crate::strpool_str!("wd"));
        }
        // else if chr_code=height_offset then print_esc("ht")
        else if $chr_code.get() == height_offset as chr_code_repr {
            print_esc($globals, crate::strpool_str!("ht"));
        }
        // else print_esc("dp");
        else {
            print_esc($globals, crate::strpool_str!("dp"));
        }
        use crate::section_0135::height_offset;
        use crate::section_0135::width_offset;
        use crate::section_0297::chr_code_repr;
        true
    }
    // last_item: case chr_code of
    else if $cmd == last_item {
        // int_val: print_esc("lastpenalty");
        if $chr_code.get() == last_item_command_kind::int_val as chr_code_repr {
            print_esc($globals, crate::strpool_str!("lastpenalty"));
        }
        // dimen_val: print_esc("lastkern");
        else if $chr_code.get() == last_item_command_kind::dimen_val as chr_code_repr {
            print_esc($globals, crate::strpool_str!("lastkern"));
        }
        // glue_val: print_esc("lastskip");
        else if $chr_code.get() == last_item_command_kind::glue_val as chr_code_repr {
            print_esc($globals, crate::strpool_str!("lastskip"));
        }
        // input_line_no_code: print_esc("inputlineno");
        else if $chr_code.get() == last_item_command_kind::input_line_no_code as chr_code_repr {
            print_esc($globals, crate::strpool_str!("inputlineno"));
        }
        // othercases print_esc("badness")
        else {
            print_esc($globals, crate::strpool_str!("badness"));
        }
        // endcases;
        use crate::section_0297::chr_code_repr;
        use crate::section_0416::last_item_command_kind;
        true
    } else {
        false
    };
    use crate::section_0063::print_esc;
    use crate::section_0208::*;
    use crate::section_0209::*;
    processed
}}
