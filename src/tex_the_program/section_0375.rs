//! @ An |end_template| command is effectively changed to an |endv| command
//! by the following code. (The reason for this is discussed below; the
//! |frozen_end_template| at the end of the template has passed the
//! |check_outer_validity| test, so its mission of error detection has been
//! accomplished.)
//
// @<Insert a token containing |frozen_endv|@>=
pub(crate) macro Insert_a_token_containing_frozen_endv($globals:expr) {{
    // begin cur_tok:=cs_token_flag+frozen_endv; back_input;
    $globals.cur_tok = cur_tok_type::from_cs(frozen_endv as _);
    back_input($globals);
    // end
    use crate::section_0222::frozen_endv;
    use crate::section_0297::cur_tok_type;
    use crate::section_0325::back_input;
}}
