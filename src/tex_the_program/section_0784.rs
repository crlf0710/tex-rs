//! ` `
// @<Scan the template \<v_j>...@>=
pub(crate) macro Scan_the_template_v_j__putting_the_resulting_token_list_in_hold_head {
    ($globals:expr) => {{
        /// for short-term temporary use
        let mut p: pointer;
        // p:=hold_head; link(p):=null;
        p = hold_head;
        link!($globals, p) = null;
        crate::region_forward_label!(
        |'done2|
        {
        // loop@+  begin continue: get_preamble_token;
        loop {
            get_preamble_token($globals)?;
            // if (cur_cmd<=car_ret)and(cur_cmd>=tab_mark)and(align_state=-1000000) then
            //   goto done2;
            if $globals.cur_cmd <= car_ret && $globals.cur_cmd >= tab_mark && $globals.align_state == -1000000 {
                crate::goto_forward_label!('done2);
            }
            // if cur_cmd=mac_param then
            if $globals.cur_cmd == mac_param {
                todo!("error");
                // begin print_err("Only one # is allowed per tab");
                // @.Only one \# is allowed...@>
                // help3("There should be exactly one # between &'s, when an")@/
                // ("\halign or \valign is being set up. In this case you had")@/
                // ("more than one, so I'm ignoring all but the first.");
                // error; goto continue;
                // end;
            }
            // link(p):=get_avail; p:=link(p); info(p):=cur_tok;
            link!($globals, p) = get_avail($globals);
            p = link!($globals, p);
            info_tok_assign!($globals, p, $globals.cur_tok);
            // end;
        }
        // done2: link(p):=get_avail; p:=link(p);
        }
        'done2 <-
        );
        link!($globals, p) = get_avail($globals);
        p = link!($globals, p);
        // info(p):=end_template_token {put \.{\\endtemplate} at the end}
        info_tok_assign!($globals, p, cur_tok_type::new(end_template_token));
        /// put `\endtemplate` at the end
        const _: () = ();
        use crate::section_0115::pointer;
        use crate::section_0115::null;
        use crate::section_0118::link;
        use crate::section_0118::info_tok_assign;
        use crate::section_0118::info_inner;
        use crate::section_0162::hold_head;
        use crate::section_0120::get_avail;
        use crate::section_0207::car_ret;
        use crate::section_0207::tab_mark;
        use crate::section_0207::mac_param;
        use crate::section_0297::cur_tok_type;
        use crate::section_0780::end_template_token;
        use crate::section_0782::get_preamble_token;
    }}
}
