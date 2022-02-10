//! ` `
// @<Forbidden...@>=
pub(crate) macro Forbidden_cases_detected_in_main_control_1098($abs_mode_plus_cur_cmd:expr) {{
    // vmode+vadjust,
    let result = $abs_mode_plus_cur_cmd == vmode as u16 + vadjust as u16;
    use crate::section_0208::*;
    use crate::section_0211::*;
    result
}}
