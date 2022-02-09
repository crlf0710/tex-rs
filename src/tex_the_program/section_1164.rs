//! ` `
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1164($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+accent,mmode+math_accent:math_ac;
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + accent as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + math_accent as u16
    {
        math_ac($globals)?;
        true
    } else {
        false
    };
    use crate::section_0208::accent;
    use crate::section_0208::math_accent;
    use crate::section_0211::mmode;
    use crate::section_1165::math_ac;
    processed
}}
