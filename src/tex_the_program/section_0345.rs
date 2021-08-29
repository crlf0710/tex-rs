// @ @<Cases where character is ignored@>=
// any_state_plus(ignore),skip_blanks+spacer,new_line+spacer
macro_rules! State_plus_cur_cmd_matches_cases_where_character_is_ignored {
    ($state_plus_cur_cmd:expr) => {
        State_plus_cur_cmd_matches_any_case_plus!($state_plus_cur_cmd, crate::section_0207::ignore) ||
            $state_plus_cur_cmd == crate::section_0303::skip_blanks + crate::section_0207::spacer ||
            $state_plus_cur_cmd == crate::section_0303::new_line + crate::section_0207::spacer
    }
}