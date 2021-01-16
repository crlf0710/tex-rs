//! @ @<Append the current tabskip glue to the preamble list@>=
//! link(cur_align):=new_param_glue(tab_skip_code);
//! cur_align:=link(cur_align)
//!
//! @ @<Scan preamble text until |cur_cmd| is |tab_mark| or |car_ret|...@>=
//! @<Scan the template \<u_j>, putting the resulting token list in |hold_head|@>;
//! link(cur_align):=new_null_box; cur_align:=link(cur_align); {a new alignrecord}
//! info(cur_align):=end_span; width(cur_align):=null_flag;
//! u_part(cur_align):=link(hold_head);
//! @<Scan the template \<v_j>, putting the resulting token list in |hold_head|@>;
//! v_part(cur_align):=link(hold_head)
//!
