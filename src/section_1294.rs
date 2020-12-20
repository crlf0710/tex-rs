//! ` `

// @<Show the current meaning of a token...@>=
macro_rules! Show_the_current_meaning_of_a_token__then_goto_common_ending {
    ($globals:expr, $lbl_common_ending:lifetime) => {{
        // begin get_token;
        get_token($globals)?;
        // if interaction=error_stop_mode then wake_up_terminal;
        if $globals.interaction == error_stop_mode {
            wake_up_terminal($globals);
            // print_nl("> ");
            print_nl($globals, strpool_str!("> "));
            // if cur_cs<>0 then
            if $globals.cur_cs != 0 {
                // begin sprint_cs(cur_cs); print_char("=");
                sprint_cs($globals, $globals.cur_cs);
                print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b'='));
                // end;
            }
            // print_meaning; goto common_ending;
            print_meaning($globals);
            goto_forward_label!($lbl_common_ending);
            // end
        }
        
        
        use crate::section_0004::TeXGlobalsIoStringLogView;
        use crate::section_0034::wake_up_terminal;
        use crate::section_0058::print_char;
        use crate::section_0062::print_nl;
        use crate::section_0073::error_stop_mode;
        use crate::section_0263::sprint_cs;
        use crate::section_0296::print_meaning;
        use crate::section_0365::get_token;
    }}
}
