//! ` `

// @<Insert macro parameter...@>=
macro_rules! Insert_macro_parameter_and_goto_restart {
    ($globals:expr, $lbl_restart:lifetime) => {{
        // begin begin_token_list(param_stack[param_start+cur_chr-1],parameter);
        begin_token_list(
            $globals,
            $globals.param_stack[u8_from_0_to_n::new(
                (param_start!($globals) + $globals.cur_chr.get() as pointer - 1) as _,
            )],
            parameter,
        );
        // goto restart;
        goto_backward_label!($lbl_restart);
        // end
        use crate::pascal::u8_from_0_to_n;
        use crate::section_0115::pointer;
        use crate::section_0307::parameter;
        use crate::section_0323::begin_token_list;
    }};
}
