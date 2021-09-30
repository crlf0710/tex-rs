//! ` `
//! We might find a valid hit in a |y| or |z| byte that is already gone
//! from the buffer. But we can't change bytes that are gone forever; ``the
//! moving finger writes, $\ldots\,\,$.''
//
// @<Consider a node with matching width...@>=
pub(crate) macro Consider_a_node_with_matching_width__goto_found_if_it_s_a_hit($globals:expr, $mstate:expr, $p:expr, $lbl_found:lifetime, $lbl_not_found:lifetime) {{
    // case mstate+info(p) of
    let mstate_plus_info_p = $mstate + info_inner!($globals, $p) as quarterword;
    // none_seen+yz_OK,none_seen+y_OK,z_seen+yz_OK,z_seen+y_OK:@t@>@;@/
    if mstate_plus_info_p == none_seen + yz_OK
        || mstate_plus_info_p == none_seen + y_OK
        || mstate_plus_info_p == z_seen + yz_OK
        || mstate_plus_info_p == z_seen + y_OK
    {
        // if location(p)<dvi_gone then goto not_found
        if location!($globals, $p) < $globals.dvi_gone {
            crate::goto_forward_label!($lbl_not_found);
        }
        // else @<Change buffered instruction to |y| or |w| and |goto found|@>;
        else {
            crate::section_0613::Change_buffered_instruction_to_y_or_w_and_goto_found!(
                $globals, $p, $lbl_found
            );
        }
    }
    // none_seen+z_OK,y_seen+yz_OK,y_seen+z_OK:@t@>@;@/
    else if mstate_plus_info_p == none_seen + z_OK
        || mstate_plus_info_p == y_seen + yz_OK
        || mstate_plus_info_p == y_seen + z_OK
    {
        // if location(p)<dvi_gone then goto not_found
        if location!($globals, $p) < $globals.dvi_gone {
            crate::goto_forward_label!($lbl_not_found);
        }
        // else @<Change buffered instruction to |z| or |x| and |goto found|@>;
        else {
            crate::section_0614::Change_buffered_instruction_to_z_or_x_and_goto_found!(
                $globals, $p, $lbl_found
            );
        }
    }
    // none_seen+y_here,none_seen+z_here,y_seen+z_here,z_seen+y_here: goto found;
    else if mstate_plus_info_p == none_seen + y_here
        || mstate_plus_info_p == none_seen + z_here
        || mstate_plus_info_p == y_seen + z_here
        || mstate_plus_info_p == z_seen + y_here
    {
        crate::goto_forward_label!($lbl_found);
    }
    // othercases do_nothing
    else {
        do_nothing!();
    }
    // endcases
    use crate::section_0016::do_nothing;
    use crate::section_0113::quarterword;
    use crate::section_0118::info_inner;
    use crate::section_0605::location;
    use crate::section_0608::y_OK;
    use crate::section_0608::y_here;
    use crate::section_0608::yz_OK;
    use crate::section_0608::z_OK;
    use crate::section_0608::z_here;
    use crate::section_0611::none_seen;
    use crate::section_0611::y_seen;
    use crate::section_0611::z_seen;
}}
