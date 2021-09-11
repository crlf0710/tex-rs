//! ` `
// @<Scan the preamble and record it in the |preamble| list@>=
pub(crate) macro Scan_the_preamble_and_record_it_in_the_preamble_list {
    ($globals:expr, $save_cs_ptr:expr) => {{
        // preamble:=null; cur_align:=align_head; cur_loop:=null; scanner_status:=aligning;
        preamble!($globals) = null;
        $globals.cur_align = align_head;
        $globals.cur_loop = null;
        $globals.scanner_status = scanner_status_kind::aligning;
        // warning_index:=save_cs_ptr; align_state:=-1000000;
        //   {at this point, |cur_cmd=left_brace|}
        $globals.warning_index = $save_cs_ptr;
        $globals.align_state = -1000000;
        crate::region_forward_label!(
        |'done|
        {
        // loop@+  begin @<Append the current tabskip glue to the preamble list@>;
        loop {
            crate::section_0778::Append_the_current_tabskip_glue_to_the_preamble_list!($globals);
            // if cur_cmd=car_ret then goto done; {\.{\\cr} ends the preamble}
            if $globals.cur_cmd == car_ret {
                /// `\cr` ends the preamble
                crate::goto_forward_label!('done);
            }
            // @<Scan preamble text until |cur_cmd| is |tab_mark| or |car_ret|,
            //   looking for changes in the tabskip glue; append an
            //   alignrecord to the preamble list@>;
            crate::section_0779::Scan_preamble_text_until_cur_cmd_is_tab_mark_or_car_ret__looking_for_changes_in_the_tabskip_glue__append_an_alignrecord_to_the_preamble_list!($globals);
            // end;
        }
        // done: scanner_status:=normal
        }
        'done <-
        );
        $globals.scanner_status = scanner_status_kind::normal;
        use crate::section_0115::null;
        use crate::section_0162::align_head;
        use crate::section_0207::car_ret;
        use crate::section_0305::scanner_status_kind;
        use crate::section_0770::preamble;
    }}
}
