//! ` `

// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1116($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // hmode+discretionary,mmode+discretionary: append_discretionary;
    let processed = if $abs_mode_plus_cur_cmd == hmode as u16 + discretionary as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + discretionary as u16
    {
        append_discretionary($globals)?;
        true
    } else {
        false
    };
    use crate::section_0208::*;
    use crate::section_0211::*;
    use crate::section_1117::append_discretionary;
    processed
}}
