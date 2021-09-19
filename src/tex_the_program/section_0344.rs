//! @ The following 48-way switch accomplishes the scanning quickly, assuming
//! that a decent \PASCAL\ compiler has translated the code. Note that the numeric
//! values for |mid_line|, |skip_blanks|, and |new_line| are spaced
//! apart from each other by |max_char_code+1|, so we can add a character's
//! command code to the state to get a single number that characterizes both.
//
// @d any_state_plus(#) == mid_line+#,skip_blanks+#,new_line+#
pub(crate) macro state_plus_cur_cmd_matches_any_case_plus($state_plus_cur_cmd:expr, $val: expr) {
    $state_plus_cur_cmd == crate::section_0303::mid_line + $val
        || $state_plus_cur_cmd == crate::section_0303::skip_blanks + $val
        || $state_plus_cur_cmd == crate::section_0303::new_line + $val
}

//
// @<Change state if necessary...@>=
pub(crate) macro Change_state_if_necessary_and_goto_switch_if_the_current_character_should_be_ignored_or_goto_reswitch_if_the_current_character_changes_to_another {
    ($globals:expr, $lbl_switch:lifetime, $lbl_reswitch:lifetime) => {{
        crate::trace_span_verbose!("Change state if...");
        // case state+cur_cmd of
        let state_plus_cur_cmd = state!($globals) + $globals.cur_cmd;
        crate::trace_expr_verbose!("state_plus_cur_cmd = {}", state_plus_cur_cmd);
        if crate::section_0345::State_plus_cur_cmd_matches_cases_where_character_is_ignored!(state_plus_cur_cmd) {
            // @<Cases where character is ignored@>: goto switch;
            crate::goto_backward_label!($lbl_switch);
        } else if state_plus_cur_cmd_matches_any_case_plus!(state_plus_cur_cmd, escape) {
            // any_state_plus(escape): @<Scan a control sequence
            //   and set |state:=skip_blanks| or |mid_line|@>;
            crate::section_0354::Scan_a_control_sequence_and_set_state_skip_blanks_or_mid_line!($globals);
        }
        else if state_plus_cur_cmd_matches_any_case_plus!(state_plus_cur_cmd, active_char) {
            // any_state_plus(active_char): @<Process an active-character control sequence
            //   and set |state:=mid_line|@>;
            crate::section_0353::Process_an_active_character_control_sequence_and_set_state_mid_line!($globals);
        }
        else if state_plus_cur_cmd_matches_any_case_plus!(state_plus_cur_cmd, sup_mark) {
            // any_state_plus(sup_mark): @<If this |sup_mark| starts an expanded character
            //   like~\.{\^\^A} or~\.{\^\^df}, then |goto reswitch|,
            //   otherwise set |state:=mid_line|@>;
            crate::section_0352::If_this_sup_mark_starts_an_expanded_character_like___A__or__df__then_goto_reswitch__otherwise_set_state__mid_line!
                ($globals, $lbl_reswitch);
        }
        else if state_plus_cur_cmd_matches_any_case_plus!(state_plus_cur_cmd, invalid_char) {
            // any_state_plus(invalid_char): @<Decry the invalid character and
            //   |goto restart|@>;
            todo!("invalid_char");
        }
        // @t\4@>@<Handle situations involving spaces, braces, changes of state@>@;
        else if crate::section_0347::Handle_situations_involving_spaces_braces_changes_of_state!(
            $globals, state_plus_cur_cmd, $lbl_switch) {
            // already handled
        }
        else {
            // othercases do_nothing
            do_nothing!();
        }
        // endcases
        use crate::section_0016::do_nothing;
        use crate::section_0303::mid_line;
        use crate::section_0207::car_ret;
        use crate::section_0207::escape;
        use crate::section_0207::spacer;
        use crate::section_0207::comment;
        use crate::section_0207::sup_mark;
        use crate::section_0207::active_char;
        use crate::section_0207::left_brace;
        use crate::section_0207::right_brace;
        use crate::section_0207::other_char;
        use crate::section_0207::letter;
        use crate::section_0207::sub_mark;
        use crate::section_0207::tab_mark;
        use crate::section_0207::mac_param;
        use crate::section_0207::math_shift;
        use crate::section_0207::invalid_char;
        use crate::section_0210::outer_call;
        use crate::section_0222::active_base;
        use crate::section_0297::chr_code_type;
        use crate::section_0302::state;
        use crate::section_0303::skip_blanks;
        use crate::section_0336::check_outer_validity;
    }}
}
