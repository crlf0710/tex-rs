//! ` `

// @<Scan for \(u)units that are internal dimensions...@>=
pub(crate) macro Scan_for_u_units_that_are_internal_dimensions__goto_attach_sign_with_cur_val_set_if_found {
    ($globals:expr, $f:expr, $mu:expr, $lbl_attach_sign:lifetime) => {{
        /// temporary storage of `cur_val`
        let save_cur_val:integer;
        // save_cur_val:=cur_val;
        save_cur_val = $globals.cur_val;
        // @<Get the next non-blank non-call...@>;
        crate::section_0406::Get_the_next_non_blank_non_call_token!($globals);
        crate::region_forward_label!(
        |'not_found|
        {
        let v;
        crate::region_forward_label!(
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
                scan_something_internal($globals,
                    small_number::new(cur_val_level_kind::dimen_val as _), false)?;
            }
            // v:=cur_val; goto found;
            v = $globals.cur_val;
            crate::goto_forward_label!('found);
            // end;
        }
        // if mu then goto not_found;
        if $mu {
            crate::goto_forward_label!('not_found);
        }
        // if scan_keyword("em") then v:=(@<The em width for |cur_font|@>)
        if scan_keyword($globals, crate::strpool_str!("em"))? {
            v = crate::section_0558::The_em_width_for_cur_font!($globals).inner();
        }
        // @.em@>
        // else if scan_keyword("ex") then v:=(@<The x-height for |cur_font|@>)
        else if scan_keyword($globals, crate::strpool_str!("ex"))? {
            v = crate::section_0559::The_x_height_for_cur_font!($globals).inner();
        }
        // @.ex@>
        // else goto not_found;
        else {
            crate::goto_forward_label!('not_found);
        }
        // @<Scan an optional space@>;
        crate::section_0443::Scan_an_optional_space!($globals);
        }
        'found <-
        );
        // found:cur_val:=nx_plus_y(save_cur_val,v,xn_over_d(v,f,@'200000));
        let x = xn_over_d($globals, scaled::new_from_inner(v), $f, 0o200000);
        $globals.cur_val = nx_plus_y!($globals, save_cur_val, scaled::new_from_inner(v),
            x).inner();
        // goto attach_sign;
        crate::goto_forward_label!($lbl_attach_sign);
        }
        // not_found:
        'not_found <-
        );
        use crate::pascal::integer;
        use crate::section_0101::scaled;
        use crate::section_0101::small_number;
        use crate::section_0105::nx_plus_y;
        use crate::section_0107::xn_over_d;
        use crate::section_0325::back_input;
        use crate::section_0407::scan_keyword;
        use crate::section_0410::cur_val_level_kind;
        use crate::section_0413::scan_something_internal;
        use crate::section_0208::min_internal;
        use crate::section_0209::max_internal;
    }}
}
