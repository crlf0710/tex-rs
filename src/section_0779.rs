//! ` `
// @<Scan preamble text until |cur_cmd| is |tab_mark| or |car_ret|...@>=
macro_rules! Scan_preamble_text_until_cur_cmd_is_tab_mark_or_car_ret__looking_for_changes_in_the_tabskip_glue__append_an_alignrecord_to_the_preamble_list {
    ($globals:expr) => {{
        // @<Scan the template \<u_j>, putting the resulting token list in |hold_head|@>;
        Scan_the_template_u_j__putting_the_resulting_token_list_in_hold_head!($globals);
        // link(cur_align):=new_null_box; cur_align:=link(cur_align); {a new alignrecord}
        /// a new alignrecord
        const _ : () = ();
        link!($globals, $globals.cur_align) = new_null_box($globals)?;
        $globals.cur_align = link!($globals, $globals.cur_align);
        // info(cur_align):=end_span; width(cur_align):=null_flag;
        info_inner!($globals, $globals.cur_align) = end_span;
        width!($globals, $globals.cur_align) = null_flag;
        // u_part(cur_align):=link(hold_head);
        u_part!($globals, $globals.cur_align) = link!($globals, hold_head as pointer) as _;
        // @<Scan the template \<v_j>, putting the resulting token list in |hold_head|@>;
        Scan_the_template_v_j__putting_the_resulting_token_list_in_hold_head!($globals);
        // v_part(cur_align):=link(hold_head)
        v_part!($globals, $globals.cur_align) = link!($globals, hold_head as pointer) as _;
        use crate::section_0136::new_null_box;
        use crate::section_0138::null_flag;
        use crate::section_0162::end_span;
        use crate::section_0162::hold_head;
    }}
}
