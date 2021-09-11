//! @ The \.{\\uppercase} and \.{\\lowercase} commands are implemented by
//! building a token list and then changing the cases of the letters in it.
//
// @<Cases of |main_control| that don't...@>=
pub(crate) macro Cases_of_main_control_that_dont_depend_on_mode_1285($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // any_mode(case_shift):shift_case;
    let processed =
        if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, case_shift as u16) {
            shift_case($globals)?;
            use crate::section_1288::shift_case;
            true
        } else {
            false
        };
    use crate::section_0208::*;
    use crate::section_1045::abs_mode_plus_cur_cmd_matches_any_mode;
    processed
}}
