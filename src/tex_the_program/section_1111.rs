//! ` `
// @<Forbidden...@>=vmode+ital_corr,
pub(crate) macro Forbidden_cases_detected_in_main_control_1111($abs_mode_plus_cur_cmd:expr) {{
    let result = $abs_mode_plus_cur_cmd == vmode as u16 + ital_corr as u16;
    use crate::section_0208::*;
    use crate::section_0211::*;
    result
}}
