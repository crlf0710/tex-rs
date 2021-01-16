//! ` `
// @<Scan the preamble and record it in the |preamble| list@>=
macro_rules! Scan_the_preamble_and_record_it_in_the_preamble_list {
    ($globals:expr) => {{
        // preamble:=null; cur_align:=align_head; cur_loop:=null; scanner_status:=aligning;
        // warning_index:=save_cs_ptr; align_state:=-1000000;
        //   {at this point, |cur_cmd=left_brace|}
        // loop@+  begin @<Append the current tabskip glue to the preamble list@>;
        //   if cur_cmd=car_ret then goto done; {\.{\\cr} ends the preamble}
        //   @<Scan preamble text until |cur_cmd| is |tab_mark| or |car_ret|,
        //     looking for changes in the tabskip glue; append an
        //     alignrecord to the preamble list@>;
        //   end;
        // done: scanner_status:=normal
    }}
}
