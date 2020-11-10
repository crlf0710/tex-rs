//! @ Penalty nodes get into a list via the |break_penalty| command.
//! @^penalties@>
//
// @<Cases of |main_control| that build...@>=
macro_rules! Cases_of_main_control_that_build_boxes_and_lists_1102 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        trace_span!("Cases of `main_control` that build...1102");
        // any_mode(break_penalty): append_penalty;
        if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, break_penalty as u16) {
            append_penalty($globals);
            use crate::section_1103::append_penalty;
            true
        } else {
            false
        }
    }}
}
