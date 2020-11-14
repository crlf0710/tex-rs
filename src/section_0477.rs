//! ` `

// @<Scan and build the body of the token list; |goto found| when finished@>=
macro_rules! Scan_and_build_the_body_of_the_token_list__goto_found_when_finished {
    ($globals:expr, $macro_def:expr, $xpand:expr, $unbalance:expr, $p:expr, $q:expr, $lbl_found:lifetime) => {
        trace_span!("Scan and build the body of the token list...");
        // unbalance:=1;
        $unbalance = 1;
        // loop@+  begin if xpand then @<Expand the next part of the input@>
        loop {
            if $xpand {
                Expand_the_next_part_of_the_input!($globals, $p, $q);
            }
            // else get_token;
            else {
                get_token($globals)?;
            }
            // if cur_tok<right_brace_limit then
            if $globals.cur_tok < right_brace_limit {
                // if cur_cmd<right_brace then incr(unbalance)
                if $globals.cur_cmd < right_brace {
                    incr!($unbalance);
                }
                // else  begin decr(unbalance);
                else {
                    decr!($unbalance);
                    // if unbalance=0 then goto found;
                    if $unbalance == 0 {
                        goto_forward_label!($lbl_found);
                    }
                    // end
                }
            }
            // else if cur_cmd=mac_param then
            else if $globals.cur_cmd == mac_param {
                // if macro_def then @<Look for parameter number or \.{\#\#}@>;
                if $macro_def {
                    Look_for_parameter_number_or_sharpsharp!($globals, $xpand);
                }
            }
            // store_new_token(cur_tok);
            store_new_token!($globals, $globals.cur_tok.get(), $p, $q);
            // end
        }

        use crate::section_0207::mac_param;
        use crate::section_0207::right_brace;
        use crate::section_0289::right_brace_limit;
        use crate::section_0365::get_token;
    }
}