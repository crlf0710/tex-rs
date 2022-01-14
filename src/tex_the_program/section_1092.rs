//! ` `

// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1092($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // hmode+start_par,mmode+start_par: indent_in_hmode;
    let processed = if $abs_mode_plus_cur_cmd == hmode as u16 + start_par as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + start_par as u16
    {
        todo!("indent_in_hmode");
        true
    } else {
        false
    };
    use crate::section_0208::start_par;
    use crate::section_0211::hmode;
    use crate::section_0211::mmode;
    processed
}}
