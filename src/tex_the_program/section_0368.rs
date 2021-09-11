//! @ It takes only a little shuffling to do what \TeX\ calls \.{\\expandafter}.
//
// @<Expand the token after...@>=
pub(crate) macro Expand_the_token_after_the_next_token($globals:expr) {{
    /// token that is being "expanded after"
    let t;

    // begin get_token; t:=cur_tok; get_token;
    get_token($globals)?;
    t = $globals.cur_tok;
    get_token($globals)?;
    // if cur_cmd>max_command then expand@+else back_input;
    if $globals.cur_cmd > max_command {
        expand($globals)?;
    } else {
        back_input($globals);
    }
    // cur_tok:=t; back_input;
    $globals.cur_tok = t;
    back_input($globals);
    // end

    use crate::section_0209::max_command;
    use crate::section_0325::back_input;
    use crate::section_0365::get_token;
    use crate::section_0366::expand;
}}
