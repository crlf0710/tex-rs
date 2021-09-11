//! @ When an |extension| command occurs in |main_control|, in any mode,
//! the |do_extension| routine is called.
//
// @<Cases of |main_control| that are for extensions...@>=
pub(crate) macro Cases_of_main_control_that_are_for_extensions_to_TeX_1347($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // any_mode(extension):do_extension;
    let processed =
        if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, extension as u16) {
            do_extension($globals)?;
            use crate::section_1348::do_extension;
            true
        } else {
            false
        };
    use crate::section_0208::*;
    use crate::section_1045::abs_mode_plus_cur_cmd_matches_any_mode;
    processed
}}
