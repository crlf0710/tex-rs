//! @ The user can issue messages to the terminal, regardless of the
//! current mode.
//
// @<Cases of |main_control| that don't...@>=
macro_rules! Cases_of_main_control_that_dont_depend_on_mode_1276 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        // any_mode(message):issue_message;
        if false ||
            abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, message as u16) ||
            false {
            issue_message($globals);
            use crate::section_1279::issue_message;
            true
        } else {
            false
        }
    }}
}
