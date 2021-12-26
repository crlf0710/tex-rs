//! ` `

// @<Cases of |main_control| that don't...@>=
pub(crate) macro Cases_of_main_control_that_dont_depend_on_mode_1271($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // any_mode(after_group):begin get_token; save_for_after(cur_tok);
    let processed =
        if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, after_group as u16) {
            get_token($globals)?;
            save_for_after($globals, $globals.cur_tok.get());
            // end;
            true
        } else {
            false
        };
    use crate::section_0208::after_group;
    use crate::section_0280::save_for_after;
    use crate::section_0365::get_token;
    use crate::section_1045::abs_mode_plus_cur_cmd_matches_any_mode;
    processed
}}
