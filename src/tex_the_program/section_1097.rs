//! @ Insertion and adjustment and mark nodes are constructed by the following
//! pieces of the program.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1097($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // any_mode(insert),hmode+vadjust,mmode+vadjust: begin_insert_or_adjust;
    let processed =
        if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, insert as u16)
            || $abs_mode_plus_cur_cmd == hmode as u16 + vadjust as u16
            || $abs_mode_plus_cur_cmd == mmode as u16 + vadjust as u16
        {
            begin_insert_or_adjust($globals)?;
            true
        }
        // any_mode(mark): make_mark;
        else if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, mark as u16) {
            todo!("make_mark");
            true
        } else {
            false
        };
    use crate::section_0208::*;
    use crate::section_0211::*;
    use crate::section_1045::abs_mode_plus_cur_cmd_matches_any_mode;
    use crate::section_1099::begin_insert_or_adjust;
    processed
}}
