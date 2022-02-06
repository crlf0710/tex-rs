//! ` `

// @<Cases of |handle_right_brace|...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1173($globals:expr) {{
    // math_choice_group: build_choices;
    let processed = if $globals.cur_group == math_choice_group {
        build_choices($globals)?;
        true
    } else {
        false
    };
    use crate::section_0269::math_choice_group;
    use crate::section_1174::build_choices;
    processed
}}
