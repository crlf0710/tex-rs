//! @ The |space_factor| or |prev_depth| settings are changed when a |set_aux|
//! command is sensed. Similarly, |prev_graf| is changed in the presence of
//! |set_prev_graf|, and |dead_cycles| or |insert_penalties| in the presence of
//! |set_page_int|. These definitions are always global.
//!
//! When some dimension of a box register is changed, the change isn't exactly
//! global; but \TeX\ does not look at the \.{\\global} switch.
//
// @<Assignments@>=
pub(crate) macro Assignments_1242($globals:expr, $cur_cmd:expr, $a:expr) {{
    // set_aux:alter_aux;
    let processed = if $cur_cmd == set_aux {
        alter_aux($globals)?;
        use crate::section_1243::alter_aux;
        true
    }
    // set_prev_graf:alter_prev_graf;
    else if $cur_cmd == set_prev_graf {
        alter_prev_graf($globals)?;
        use crate::section_1244::alter_prev_graf;
        true
    }
    // set_page_dimen:alter_page_so_far;
    else if $cur_cmd == set_page_dimen {
        alter_page_so_far($globals)?;
        use crate::section_1245::alter_page_so_far;
        true
    }
    // set_page_int:alter_integer;
    else if $cur_cmd == set_page_int {
        alter_integer($globals)?;
        use crate::section_1246::alter_integer;
        true
    }
    // set_box_dimen:alter_box_dimen;
    else if $cur_cmd == set_box_dimen {
        alter_box_dimen($globals)?;
        use crate::section_1247::alter_box_dimen;
        true
    } else {
        false
    };
    use crate::section_0209::*;
    processed
}}
