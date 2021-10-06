//! ` `

// @<Cases of |handle_right_brace|...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1133($globals:expr) {{
    // no_align_group: begin end_graf; unsave; align_peek;
    let processed = if $globals.cur_group == no_align_group {
        todo!("no_align_group");
        // end;
        true
    } else {
        false
    };
    use crate::section_0269::no_align_group;
    processed
}}
