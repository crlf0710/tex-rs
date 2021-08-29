//! ` `

// @<Store the current token, but |goto continue| if it is...@>=
macro_rules! Store_the_current_token__but_goto_continue_if_it_is_a_blank_space_that_would_become_an_undelimited_parameter {
    ($globals:expr, $info_r:expr, $p:expr, $q:expr, $lbl_continue:lifetime) => {{
        // begin if cur_tok=space_token then
        if $globals.cur_tok == space_token {
            // if info(r)<=end_match_token then
            //   if info(r)>=match_token then goto continue;
            if $info_r <= end_match_token && $info_r >= match_token {
                goto_backward_label!($lbl_continue);
            }
        }
        // store_new_token(cur_tok);
        store_new_token!($globals, $globals.cur_tok.get(), $p, $q);
        // end
        use crate::section_0289::space_token;
    }}
}
