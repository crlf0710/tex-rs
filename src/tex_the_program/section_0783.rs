//! @ Spaces are eliminated from the beginning of a template.
//
// @<Scan the template \<u_j>...@>=
pub(crate) macro Scan_the_template_u_j__putting_the_resulting_token_list_in_hold_head {
    ($globals:expr) => {{
        /// for short-term temporary use
        let p: pointer;
        // p:=hold_head; link(p):=null;
        p = hold_head;
        link!($globals, p) = null;
        crate::region_forward_label!(
        |'done1|
        {
        // loop@+  begin get_preamble_token;
        loop {
            get_preamble_token($globals)?;
            // if cur_cmd=mac_param then goto done1;
            if $globals.cur_cmd == mac_param {
                crate::goto_forward_label!('done1);
            }
            todo!("scan u_j");
            // if (cur_cmd<=car_ret)and(cur_cmd>=tab_mark)and(align_state=-1000000) then
            //  if (p=hold_head)and(cur_loop=null)and(cur_cmd=tab_mark)
            //   then cur_loop:=cur_align
            //  else  begin print_err("Missing # inserted in alignment preamble");
            // @.Missing \# inserted...@>
            //   help3("There should be exactly one # between &'s, when an")@/
            //   ("\halign or \valign is being set up. In this case you had")@/
            //   ("none, so I've put one in; maybe that will work.");
            //   back_error; goto done1;
            //   end
            // else if (cur_cmd<>spacer)or(p<>hold_head) then
            //   begin link(p):=get_avail; p:=link(p); info(p):=cur_tok;
            //   end;
            // end;
        }
        // done1:
        }
        'done1 <-
        );
        use crate::section_0115::null;
        use crate::section_0115::pointer;
        use crate::section_0118::link;
        use crate::section_0162::hold_head;
        use crate::section_0207::mac_param;
        use crate::section_0782::get_preamble_token;
    }}
}
