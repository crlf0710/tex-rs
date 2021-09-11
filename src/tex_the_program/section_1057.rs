//! @ The processing of things like \.{\\hskip} and \.{\\vskip} is slightly
//! more complicated. But the code in |main_control| is very short, since
//! it simply calls on the action routine |append_glue|. Similarly, \.{\\kern}
//! activates |append_kern|.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1057($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // vmode+vskip,hmode+hskip,mmode+hskip,mmode+mskip: append_glue;
    let processed = if $abs_mode_plus_cur_cmd == vmode as u16 + vskip as u16
        || $abs_mode_plus_cur_cmd == hmode as u16 + hskip as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + hskip as u16
        || $abs_mode_plus_cur_cmd == mmode as u16 + mskip as u16
    {
        append_glue($globals)?;
        use crate::section_1060::append_glue;
        true
    }
    // any_mode(kern),mmode+mkern: append_kern;
    else if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, kern as u16)
        || $abs_mode_plus_cur_cmd == mmode as u16 + mkern as u16
    {
        append_kern($globals);
        use crate::section_1061::append_kern;
        true
    } else {
        false
    };
    use crate::section_0208::*;
    use crate::section_0211::*;
    use crate::section_1045::abs_mode_plus_cur_cmd_matches_any_mode;
    processed
}}
