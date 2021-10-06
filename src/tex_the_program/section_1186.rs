//! ` `
//! Now at last we're ready to see what happens when a right brace occurs
//! in a math formula. Two special cases are simplified here: Braces are effectively
//! removed when they surround a single Ord without sub/superscripts, or when they
//! surround an accent that is the nucleus of an Ord atom.

// @<Cases of |handle...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1186($globals:expr) {{
    // math_group: begin unsave; decr(save_ptr);@/
    let processed = if $globals.cur_group == math_group {
        unsave($globals)?;
        decr!($globals.save_ptr);
        // math_type(saved(0)):=sub_mlist; p:=fin_mlist(null); info(saved(0)):=p;
        // if p<>null then if link(p)=null then
        //  if type(p)=ord_noad then
        //   begin if math_type(subscr(p))=empty then
        //    if math_type(supscr(p))=empty then
        //     begin mem[saved(0)].hh:=mem[nucleus(p)].hh;
        //     free_node(p,noad_size);
        //     end;
        //   end
        // else if type(p)=accent_noad then if saved(0)=nucleus(tail) then
        //  if type(tail)=ord_noad then @<Replace the tail of the list by |p|@>;
        // end;
        todo!("math_group");
        true
    } else {
        false
    };
    use crate::section_0016::decr;
    use crate::section_0269::math_group;
    use crate::section_0281::unsave;
    processed
}}
