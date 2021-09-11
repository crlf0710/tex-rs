//! ` `

// @<Contribute an entire group to the current parameter@>=
pub(crate) macro Contribute_an_entire_group_to_the_current_parameter {
    ($globals:expr, $p:expr, $q:expr, $unbalance:expr, $rbrace_ptr:expr) => {{
        // begin unbalance:=1;
        $unbalance = 1;
        crate::region_forward_label!(
            |'done1|
            {

                // @^inner loop@>
                // loop@+  begin fast_store_new_token(cur_tok); get_token;
                loop {
                    fast_store_new_token!($globals, $globals.cur_tok, $p, $q);
                    get_token($globals)?;
                    // if cur_tok=par_token then if long_state<>long_call then
                    if $globals.cur_tok == $globals.par_token {
                        if $globals.long_state != long_call {
                            // @<Report a runaway argument and abort@>;
                            todo!("report a runaway...");
                        }
                    }
                    // if cur_tok<right_brace_limit then
                    if $globals.cur_tok < right_brace_limit {
                        // if cur_tok<left_brace_limit then incr(unbalance)
                        if $globals.cur_tok < left_brace_limit {
                            incr!($unbalance);
                        }
                        // else  begin decr(unbalance);
                        else {
                            decr!($unbalance);
                            // if unbalance=0 then goto done1;
                            if $unbalance == 0 {
                                crate::goto_forward_label!('done1);
                            }
                            // end;
                        }
                    }
                    // end;
                }

            }
            // done1: rbrace_ptr:=p; store_new_token(cur_tok);
            'done1 <-
        );
        $rbrace_ptr = $p;
        store_new_token!($globals, $globals.cur_tok.get(), $p, $q);
        // end
        use crate::section_0016::decr;
        use crate::section_0016::incr;
        use crate::section_0210::*;
        use crate::section_0289::left_brace_limit;
        use crate::section_0289::right_brace_limit;
        use crate::section_0365::get_token;
        use crate::section_0371::store_new_token;
        use crate::section_0371::fast_store_new_token;
    }}
}
