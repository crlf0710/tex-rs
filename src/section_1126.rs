//! @ When `\.{\\cr}' or `\.{\\span}' or a tab mark comes through the scanner
//! into |main_control|, it might be that the user has foolishly inserted
//! one of them into something that has nothing to do with alignment. But it is
//! far more likely that a left brace or right brace has been omitted, since
//! |get_next| takes actions appropriate to alignment only when `\.{\\cr}'
//! or `\.{\\span}' or tab marks occur with |align_state=0|. The following
//! program attempts to make an appropriate recovery.
//
// @<Cases of |main_control| that build...@>=
macro_rules! Cases_of_main_control_that_build_boxes_and_lists_1126 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        // any_mode(car_ret), any_mode(tab_mark): align_error;
        if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, car_ret as u16) || 
            abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, tab_mark as u16) {
            todo!("align_error");
            true
        }
        // any_mode(no_align): no_align_error;
        else if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, no_align as u16) {
            todo!("no_align_error");
            true
        }
        // any_mode(omit): omit_error;
        else if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, omit as u16) {
            todo!("omit_error");
            true
        } else {
            false
        }
    }}
}
