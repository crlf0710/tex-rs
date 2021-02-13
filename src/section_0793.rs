//! @ @<Lengthen the preamble...@>=
//! begin link(q):=new_null_box; p:=link(q); {a new alignrecord}
//! info(p):=end_span; width(p):=null_flag; cur_loop:=link(cur_loop);
//! @<Copy the templates from node |cur_loop| into node |p|@>;
//! cur_loop:=link(cur_loop);
//! link(p):=new_glue(glue_ptr(cur_loop));
//! subtype(link(p)):=tab_skip_code+1;
//! end
//!
