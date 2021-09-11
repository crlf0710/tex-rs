//! ` `

// @<Scan and build the parameter part...@>=
pub(crate) macro Scan_and_build_the_parameter_part_of_the_macro_definition {
    ($globals:expr, $t:expr, $hash_brace:expr, $p:expr, $q:expr) => {{
        crate::region_forward_label! {
        |'done|
        {
        crate::region_forward_label! {
        |'done1|
        {
            // begin loop begin continue: get_token; {set |cur_cmd|, |cur_chr|, |cur_tok|}
            loop {
                crate::region_backward_label!(
                    'continue_ <-
                    {
                        /// set `cur_cmd`, `cur_chr`, `cur_tok`
                        get_token($globals)?;
                        // if cur_tok<right_brace_limit then goto done1;
                        if $globals.cur_tok < right_brace_limit {
                            crate::goto_forward_label!('done1);
                        }
                        // if cur_cmd=mac_param then
                        if $globals.cur_cmd == mac_param {
                            // @<If the next character is a parameter number, make |cur_tok|
                            //   a |match| token; but if it is a left brace, store
                            //   `|left_brace|, |end_match|', set |hash_brace|, and |goto done|@>;
                            crate::section_0476::If_the_next_character_is_a_parameter_number__make_cur_tok_a_match_token__but_if_it_is_a_left_brace__store_left_brace_end_match__set_hash_brace_and_goto_done!
                                ($globals, $t, $hash_brace, 'done, 'continue_, $p, $q);
                        }
                        // store_new_token(cur_tok);
                        store_new_token!($globals, $globals.cur_tok.get(), $p, $q);
                        // end;
                    }
                    |'continue_|
                );
            }
        }
        // done1: store_new_token(end_match_token);
        'done1 <-
        }
        store_new_token!($globals, end_match_token, $p, $q);
        // if cur_cmd=right_brace then
        if $globals.cur_cmd == right_brace {
            // @<Express shock at the missing left brace; |goto found|@>;
            todo!("shock")
        }
        }
        // done: end
        'done <-
        }
        use crate::section_0207::mac_param;
        use crate::section_0207::right_brace;
        use crate::section_0289::end_match_token;
        use crate::section_0289::right_brace_limit;
        use crate::section_0365::get_token;
        use crate::section_0371::store_new_token;
    }}
}
