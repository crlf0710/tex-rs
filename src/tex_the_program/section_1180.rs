//! ` `
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1180($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+above: math_fraction;
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + above as u16 {
        math_fraction($globals)?;
        true
    } else {
        false
    };
    use crate::section_0208::above;
    use crate::section_0211::mmode;
    use crate::section_1181::math_fraction;
    processed
}}
