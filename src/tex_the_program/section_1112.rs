//! @ Italic corrections are converted to kern nodes when the |ital_corr| command
//! follows a character. In math mode the same effect is achieved by appending
//! a kern of zero here, since italic corrections are supplied later.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1112($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // hmode+ital_corr: append_italic_correction;
    let processed = if $abs_mode_plus_cur_cmd == hmode as u16 + ital_corr as u16 {
        append_italic_correction($globals)?;
        true
    }
    // mmode+ital_corr: tail_append(new_kern(0));
    else if $abs_mode_plus_cur_cmd == mmode as u16 + ital_corr as u16 {
        todo!("tail_append");
        true
    } else {
        false
    };
    use crate::section_0208::ital_corr;
    use crate::section_0211::*;
    use crate::section_1113::append_italic_correction;
    processed
}}
