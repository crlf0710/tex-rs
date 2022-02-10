//! @ We get into ordinary math mode from display math mode when `\.{\\eqno}' or
//! `\.{\\leqno}' appears. In such cases |cur_chr| will be 0 or~1, respectively;
//! the value of |cur_chr| is placed onto |save_stack| for safe keeping.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1140($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+eq_no: if privileged then
    // if cur_group=math_shift_group then start_eq_no
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + eq_no as u16 {
        if privileged($globals)? {
            if $globals.cur_group == math_shift_group {
                start_eq_no($globals);
            }
            // else off_save;
            else {
                off_save($globals);
            }
        }
        true
    } else {
        false
    };
    use crate::section_0208::*;
    use crate::section_0211::*;
    use crate::section_0269::math_shift_group;
    use crate::section_1051::privileged;
    use crate::section_1064::off_save;
    use crate::section_1142::start_eq_no;
    processed
}}
