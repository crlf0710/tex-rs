//! ` `

// @<Get the next non-blank non-call token@>=
pub(crate) macro Get_the_next_non_blank_non_call_token($globals:expr) {{
    // repeat get_x_token;
    loop {
        get_x_token($globals)?;
        // until cur_cmd<>spacer
        if $globals.cur_cmd != spacer {
            break;
        }
    }
    use crate::section_0207::spacer;
    use crate::section_0380::get_x_token;
}}
