//! @ The following 48-way switch accomplishes the scanning quickly, assuming
//! that a decent \PASCAL\ compiler has translated the code. Note that the numeric
//! values for |mid_line|, |skip_blanks|, and |new_line| are spaced
//! apart from each other by |max_char_code+1|, so we can add a character's
//! command code to the state to get a single number that characterizes both.
//
// @d any_state_plus(#) == mid_line+#,skip_blanks+#,new_line+#
macro_rules! State_plus_cur_cmd_matches_any_case_plus {
    ($state_plus_cur_cmd:expr, $val: expr) => {
        $state_plus_cur_cmd == crate::section_0303::mid_line + $val ||
        $state_plus_cur_cmd == crate::section_0303::skip_blanks + $val ||
        $state_plus_cur_cmd == crate::section_0303::new_line + $val
    }
}

//
// @<Change state if necessary...@>=
macro_rules! Change_state_if_necessary_and_goto_switch_if_the_current_character_should_be_ignored_or_goto_reswitch_if_the_current_character_changes_to_another {
    ($globals:expr, $lbl_switch:lifetime) => {{
        // case state+cur_cmd of
        let state_plus_cur_cmd = state!($globals) + $globals.cur_cmd;
        if State_plus_cur_cmd_matches_cases_where_character_is_ignored!(state_plus_cur_cmd) {
            // @<Cases where character is ignored@>: goto switch;
            goto_backward_label!($lbl_switch);
        } else if State_plus_cur_cmd_matches_any_case_plus!(state_plus_cur_cmd, escape) {
            // any_state_plus(escape): @<Scan a control sequence
            //   and set |state:=skip_blanks| or |mid_line|@>;
            Scan_a_control_sequence_and_set_state_skip_blanks_or_mid_line!($globals);
        }
        // any_state_plus(active_char): @<Process an active-character control sequence
        //   and set |state:=mid_line|@>;
        // any_state_plus(sup_mark): @<If this |sup_mark| starts an expanded character
        //   like~\.{\^\^A} or~\.{\^\^df}, then |goto reswitch|,
        //   otherwise set |state:=mid_line|@>;
        // any_state_plus(invalid_char): @<Decry the invalid character and
        //   |goto restart|@>;
        // @t\4@>@<Handle situations involving spaces, braces, changes of state@>@;
        else {
            // othercases do_nothing
            do_nothing!();
        }
        // endcases
        use crate::section_0207::escape;
    }}
}
