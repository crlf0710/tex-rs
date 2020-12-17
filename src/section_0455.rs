//! ` `

// @<Scan for \(u)units that are internal dimensions...@>=
macro_rules! Scan_for_u_units_that_are_internal_dimensions__goto_attach_sign_with_cur_val_set_if_found {
    ($globals:expr, $f:expr, $mu:expr, $lbl_attach_sign:lifetime) => {{
        /// temporary storage of `cur_val`
        let save_cur_val:integer;
        // save_cur_val:=cur_val;
        save_cur_val = $globals.cur_val;
        // @<Get the next non-blank non-call...@>;
        Get_the_next_non_blank_non_call_token!($globals);
        region_forward_label!(
        |'not_found|
        {
        let v;
        region_forward_label!(
        |'found|
        {
        // if (cur_cmd<min_internal)or(cur_cmd>max_internal) then back_input
        if $globals.cur_cmd < min_internal || $globals.cur_cmd > max_internal {
            back_input($globals);
        }
        // else  begin if mu then
        else {
            if $mu {
                todo!("mu")
                // begin scan_something_internal(mu_val,false); @<Coerce glue...@>;
                // if cur_val_level<>mu_val then mu_error;
                // end
            }
            // else scan_something_internal(dimen_val,false);
            else {
                todo!("scan_something_internal");
            }
            // v:=cur_val; goto found;
            v = $globals.cur_val;
            goto_forward_label!('found);
            // end;
        }
        // if mu then goto not_found;
        if $mu {
            goto_forward_label!('not_found);
        }
        // if scan_keyword("em") then v:=(@<The em width for |cur_font|@>)
        if scan_keyword($globals, strpool_str!("em"))? {
            todo!("v=em");
        }
        // @.em@>
        // else if scan_keyword("ex") then v:=(@<The x-height for |cur_font|@>)
        else if scan_keyword($globals, strpool_str!("ex"))? {
            todo!("v=ex");
        }
        // @.ex@>
        // else goto not_found;
        else {
            goto_forward_label!('not_found);
        }
        // @<Scan an optional space@>;
        Scan_an_optional_space!($globals);
        }
        'found <-
        );
        // found:cur_val:=nx_plus_y(save_cur_val,v,xn_over_d(v,f,@'200000));
        $globals.cur_val = nx_plus_y!($globals, save_cur_val, scaled::new_from_inner(v), 
            xn_over_d($globals, scaled::new_from_inner(v), $f, 0o200000)).inner();
        // goto attach_sign;
        goto_forward_label!($lbl_attach_sign);
        }
        // not_found:
        'not_found <-
        );
        use crate::section_0101::scaled;
        use crate::section_0107::xn_over_d;
    }}
}
