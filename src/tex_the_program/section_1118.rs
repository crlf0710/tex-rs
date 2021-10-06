//! ` `
//! The three discretionary lists are constructed somewhat as if they were
//! hboxes. A~subroutine called |build_discretionary| handles the transitions.
//! (This is sort of fun.)

// @<Cases of |handle...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1118($globals:expr) {{
    // disc_group: build_discretionary;
    let processed = if $globals.cur_group == disc_group {
        build_discretionary($globals)?;
        true
    } else {
        false
    };
    use crate::section_0269::disc_group;
    use crate::section_1119::build_discretionary;
    processed
}}
