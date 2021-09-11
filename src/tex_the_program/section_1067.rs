//! @ The routine for a |right_brace| character branches into many subcases,
//! since a variety of things may happen, depending on |cur_group|. Some
//! types of groups are not supposed to be ended by a right brace; error
//! messages are given in hopes of pinpointing the problem. Most branches
//! of this routine will be filled in later, when we are ready to understand
//! them; meanwhile, we must prepare ourselves to deal with such errors.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1067($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    let processed =
        if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, right_brace as u16) {
            // any_mode(right_brace): handle_right_brace;
            handle_right_brace($globals)?;
            use crate::section_1068::handle_right_brace;
            true
        } else {
            false
        };
    use crate::section_0207::*;
    use crate::section_1045::abs_mode_plus_cur_cmd_matches_any_mode;
    processed
}}
