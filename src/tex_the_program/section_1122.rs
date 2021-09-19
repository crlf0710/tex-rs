//! ` `

// We need only one more thing to complete the horizontal mode routines, namely
// the \.{\\accent} primitive.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1122($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // hmode+accent: make_accent;
    let processed = if $abs_mode_plus_cur_cmd == hmode as u16 + accent as u16 {
        make_accent($globals)?;
        true
    } else {
        false
    };
    use crate::section_0208::*;
    use crate::section_0211::*;
    use crate::section_1123::make_accent;
    processed
}}
