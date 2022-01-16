//! @ We've now covered most of the abuses of \.{\\halign} and \.{\\valign}.
//! Let's take a look at what happens when they are used correctly.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1130($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    let processed = if $abs_mode_plus_cur_cmd == vmode as u16 + halign as u16
        || $abs_mode_plus_cur_cmd == hmode as u16 + valign as u16
    {
        // vmode+halign,hmode+valign:init_align;
        init_align($globals)?;
        true
    } else if $abs_mode_plus_cur_cmd == mmode as u16 + halign as u16 {
        // mmode+halign: if privileged then
        if privileged($globals)? {
            // if cur_group=math_shift_group then init_align
            if $globals.cur_group == math_shift_group {
                init_align($globals)?;
            }
            // else off_save;
            else {
                off_save($globals);
            }
        }
        true
    } else if $abs_mode_plus_cur_cmd == vmode as u16 + endv as u16
        || $abs_mode_plus_cur_cmd == hmode as u16 + endv as u16
    {
        // vmode+endv,hmode+endv: do_endv;
        do_endv($globals)?;
        true
    } else {
        false
    };
    use crate::section_0207::*;
    use crate::section_0208::*;
    use crate::section_0211::*;
    use crate::section_0269::math_shift_group;
    use crate::section_0774::init_align;
    use crate::section_1051::privileged;
    use crate::section_1064::off_save;
    use crate::section_1131::do_endv;
    processed
}}
