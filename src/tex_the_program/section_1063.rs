//! @ If a left brace occurs in the middle of a page or paragraph, it simply
//! introduces a new level of grouping, and the matching right brace will not have
//! such a drastic effect. Such grouping affects neither the mode nor the
//! current list.
//
// @<Cases of |main_control| that build...@>=
pub(crate) macro Cases_of_main_control_that_build_boxes_and_lists_1063($globals:expr, $abs_mode_plus_cur_cmd:expr) {{
    // non_math(left_brace): new_save_level(simple_group);
    let processed = if abs_mode_plus_cur_cmd_matches_non_math_mode!(
        $abs_mode_plus_cur_cmd,
        left_brace as u16
    ) {
        new_save_level($globals, simple_group.into());
        use crate::section_0269::simple_group;
        use crate::section_0274::new_save_level;
        true
    }
    // any_mode(begin_group): new_save_level(semi_simple_group);
    else if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, begin_group as u16) {
        new_save_level($globals, semi_simple_group.into());
        use crate::section_0269::semi_simple_group;
        use crate::section_0274::new_save_level;
        true
    }
    // any_mode(end_group): if cur_group=semi_simple_group then unsave
    //   else off_save;
    else if abs_mode_plus_cur_cmd_matches_any_mode!($abs_mode_plus_cur_cmd, end_group as u16) {
        if $globals.cur_group == semi_simple_group {
            unsave($globals)?;
        } else {
            off_save($globals);
        }
        use crate::section_0269::semi_simple_group;
        use crate::section_0281::unsave;
        use crate::section_1064::off_save;
        true
    } else {
        false
    };
    use crate::section_0207::*;
    use crate::section_0208::*;
    use crate::section_1045::abs_mode_plus_cur_cmd_matches_any_mode;
    use crate::section_1046::abs_mode_plus_cur_cmd_matches_non_math_mode;
    processed
}}
