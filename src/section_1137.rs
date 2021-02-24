//! @ We get into math mode from horizontal mode when a `\.\$' (i.e., a
//! |math_shift| character) is scanned. We must check to see whether this
//! `\.\$' is immediately followed by another, in case display math mode is
//! called for.
//
// @<Cases of |main_control| that build...@>=
macro_rules! Cases_of_main_control_that_build_boxes_and_lists_1137 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        // hmode+math_shift:init_math;
        if $abs_mode_plus_cur_cmd == hmode as u16 + math_shift as u16 {
            init_math($globals)?;
            use crate::section_1138::init_math;
            true
        } else {
            false
        }
    }}
}
