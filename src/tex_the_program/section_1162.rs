//! ` `

// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1162($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // mmode+radical:math_radical;
    let processed = if $abs_mode_plus_cur_cmd == mmode as u16 + radical as u16 {
        math_radical($globals)?;
        true
    } else {
        false
    };
    use crate::section_0208::radical;
    use crate::section_0211::mmode;
    use crate::section_1163::math_radical;
    processed
}}
