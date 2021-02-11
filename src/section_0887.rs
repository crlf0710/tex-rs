//! @ The following code begins with |q| at the end of the list to be
//! justified. It ends with |q| at the beginning of that list, and with
//! |link(temp_head)| pointing to the remainder of the paragraph, if any.
//
// @<Put the \(l)\.{\\leftskip} glue at the left...@>=
macro_rules! Put_the_leftskip_glue_at_the_left_and_detach_this_line {
    ($globals:expr) => {{
        // r:=link(q); link(q):=null; q:=link(temp_head); link(temp_head):=r;
        // if left_skip<>zero_glue then
        //   begin r:=new_param_glue(left_skip_code);
        //   link(r):=q; q:=r;
        //   end
        todo!("Put the leftskip glue");
    }}
}
