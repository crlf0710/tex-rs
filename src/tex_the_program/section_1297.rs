//! ` `

// @<Show the current value of some parameter...@>=
macro_rules! Show_the_current_value_of_some_parameter_or_register__then_goto_common_ending {
    ($globals:expr, $lbl_common_ending:lifetime) => {{
        // begin p:=the_toks;
        let _p = the_toks($globals);
        // if interaction=error_stop_mode then wake_up_terminal;
        if $globals.interaction == error_stop_mode {
            wake_up_terminal($globals);
        }
        // print_nl("> "); token_show(temp_head);
        print_nl($globals, strpool_str!("> "));
        token_show($globals, temp_head);
        // flush_list(link(temp_head)); goto common_ending;
        flush_list($globals, link!($globals, temp_head));
        goto_forward_label!($lbl_common_ending);
        // end
        use crate::section_0034::wake_up_terminal;
        use crate::section_0062::print_nl;
        use crate::section_0123::flush_list;
        use crate::section_0162::temp_head;
        use crate::section_0295::token_show;
        use crate::section_0465::the_toks;
    }}
}
