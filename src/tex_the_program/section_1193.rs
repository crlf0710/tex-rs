//! @ Here is the only way out of math mode.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1193($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+math_shift: if cur_group=math_shift_group then after_math
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + math_shift as u16 {
        if $globals.cur_group == math_shift_group {
            after_math($globals);
        }
        // else off_save;
        else {
            off_save($globals);
        }
        use crate::section_0269::math_shift_group;
        use crate::section_1064::off_save;
        use crate::section_1194::after_math;
        true
    } else {
        false
    };
    use crate::section_0207::*;
    use crate::section_0211::*;
    processed
}}
