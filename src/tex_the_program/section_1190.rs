//! ` `
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1190($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+left_right: math_left_right;
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + left_right as u16 {
        math_left_right($globals)?;
        true
    } else {
        false
    };
    use crate::section_0208::left_right;
    use crate::section_0211::mmode;
    use crate::section_1191::math_left_right;
    processed
}}
