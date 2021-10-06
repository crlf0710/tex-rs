//! ` `

// @<Cases of |handle_right_brace|...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1173($globals:expr) {{
    // math_choice_group: build_choices;
    let processed = if $globals.cur_group == math_choice_group {
        todo!("build_choices");
        true
    } else {
        false
    };
    use crate::section_0269::math_choice_group;
    processed
}}
