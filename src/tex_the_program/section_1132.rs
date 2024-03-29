//! ` `
// @<Cases of |handle_right_brace|...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1132($globals:expr) {{
    // align_group: begin back_input; cur_tok:=cs_token_flag+frozen_cr;
    let processed = if $globals.cur_group == align_group {
        back_input($globals);
        $globals.cur_tok = cur_tok_type::from_cs(frozen_cr as _);
        // print_err("Missing "); print_esc("cr"); print(" inserted");
        print_err!($globals, crate::strpool_str!("Missing "));
        print_esc($globals, crate::strpool_str!("cr"));
        print($globals, crate::strpool_str!(" inserted").get() as _);
        // @.Missing \\cr inserted@>
        // help1("I'm guessing that you meant to end an alignment here.");
        help1!(
            $globals,
            crate::strpool_str!("I'm guessing that you meant to end an alignment here.")
        );
        // ins_error;
        ins_error($globals)?;
        // end;
        use crate::section_0059::print;
        use crate::section_0063::print_esc;
        use crate::section_0073::print_err;
        use crate::section_0079::help1;
        use crate::section_0222::frozen_cr;
        use crate::section_0297::cur_tok_type;
        use crate::section_0325::back_input;
        use crate::section_0327::ins_error;
        true
    } else {
        false
    };
    use crate::section_0269::*;
    processed
}}
