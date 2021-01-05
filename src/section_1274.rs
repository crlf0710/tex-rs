//! ` `

// @<Cases of |main_control| that don't...@>=
macro_rules! Cases_of_main_control_that_dont_depend_on_mode_1274 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        // any_mode(in_stream): open_or_close_in;
        if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, in_stream as u16) {
            open_or_close_in($globals);
            use crate::section_1275::open_or_close_in;
            true
        } else {
            false
        }
    }}
}
