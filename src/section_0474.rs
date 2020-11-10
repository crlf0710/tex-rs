//! ` `

// @<Scan and build the parameter part...@>=
macro_rules! Scan_and_build_the_parameter_part_of_the_macro_definition {
    ($globals:expr, $p:expr, $q:expr) => {{
        // begin loop begin get_token; {set |cur_cmd|, |cur_chr|, |cur_tok|}
        region_forward_label! {
        |'done1|
        {
            loop {
                /// set `cur_cmd`, `cur_chr`, `cur_tok`
                get_token($globals)?;
                // if cur_tok<right_brace_limit then goto done1;
                if $globals.cur_tok < right_brace_limit {
                    goto_forward_label!('done1);
                }
                // if cur_cmd=mac_param then
                if $globals.cur_cmd == mac_param {
                    // @<If the next character is a parameter number, make |cur_tok|
                    //   a |match| token; but if it is a left brace, store
                    //   `|left_brace|, |end_match|', set |hash_brace|, and |goto done|@>;
                }
                // store_new_token(cur_tok);
                store_new_token!($globals, $globals.cur_tok.get(), $p, $q);
                // end;
            }
        }
        // done1: store_new_token(end_match_token);
        'done1 <-
        }
        store_new_token!($globals, end_match_token, $p, $q);
        // if cur_cmd=right_brace then
        //   @<Express shock at the missing left brace; |goto found|@>;
        // done: end
        use crate::section_0207::mac_param;
        use crate::section_0289::end_match_token;
        use crate::section_0289::right_brace_limit;
        use crate::section_0365::get_token;
    }}
}
