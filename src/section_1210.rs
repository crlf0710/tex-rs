//! @ Every prefix, and every command code that might or might not be prefixed,
//! calls the action procedure |prefixed_command|. This routine accumulates
//! a sequence of prefixes until coming to a non-prefix, then it carries out
//! the command.
//
// @<Cases of |main_control| that don't...@>=
macro_rules! Cases_of_main_control_that_dont_depend_on_mode_1210 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        // any_mode(toks_register),
        // any_mode(assign_toks),
        // any_mode(assign_int),
        // any_mode(assign_dimen),
        // any_mode(assign_glue),
        // any_mode(assign_mu_glue),
        // any_mode(assign_font_dimen),
        // any_mode(assign_font_int),
        // any_mode(set_aux),
        // any_mode(set_prev_graf),
        // any_mode(set_page_dimen),
        // any_mode(set_page_int),
        // any_mode(set_box_dimen),
        // any_mode(set_shape),
        // any_mode(def_code),
        // any_mode(def_family),
        // any_mode(set_font),
        // any_mode(def_font),
        // any_mode(register),
        // any_mode(advance),
        // any_mode(multiply),
        // any_mode(divide),
        // any_mode(prefix),
        // any_mode(let),
        // any_mode(shorthand_def),
        // any_mode(read_to_cs),
        // any_mode(def),
        // any_mode(set_box),
        // any_mode(hyph_data),
        // any_mode(set_interaction):prefixed_command;
        if false ||
        /*abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, toks_register as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, assign_toks as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, assign_int as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, assign_dimen as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, assign_glue as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, assign_mu_glue as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, assign_font_dimen as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, assign_font_int as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, set_aux as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, set_prev_graf as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, set_page_dimen as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, set_page_int as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, set_box_dimen as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, set_shape as u16) ||
        */abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, def_code as u16) ||
        /*abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, def_family as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, set_font as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, def_font as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, register as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, advance as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, multiply as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, divide as u16) ||
        */abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, prefix as u16) ||
        /*abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, r#let as u16) ||
        */abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, shorthand_def as u16) ||
        /*abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, read_to_cs as u16) ||
        */abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, def as u16) ||
        /*abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, set_box as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, hyph_data as u16) ||
        abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, set_interaction as u16) ||
        */false {
            prefixed_command($globals)?;

            use crate::section_1211::prefixed_command;
            true
        }
        else {
            false
        }
    }}
}