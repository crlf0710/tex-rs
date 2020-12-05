//! @ We come finally to the last pieces missing from |main_control|, namely the
//! `\.{\\show}' commands that are useful when debugging.

//
// @<Cases of |main_control| that don't...@>=
macro_rules! Cases_of_main_control_that_dont_depend_on_mode_1290 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        // any_mode(xray): show_whatever;
        if false ||
            abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, xray as u16) ||
            false {
            show_whatever($globals)?;
            use crate::section_1293::show_whatever;
            true
        } else {
            false
        }
    }}
}
