//! ` `

// @<Insert a token saved by \.{\\afterassignment}, if any@>=
pub(crate) macro Insert_a_token_saved_by_afterassignment__if_any($globals:expr) {{
    // if after_token<>0 then
    if $globals.after_token != 0 {
        // begin cur_tok:=after_token; back_input; after_token:=0;
        $globals.cur_tok = $globals.after_token;
        back_input($globals);
        $globals.after_token = cur_tok_type::default();
        // end
    }
    use crate::section_0297::cur_tok_type;
    use crate::section_0325::back_input;
}}
