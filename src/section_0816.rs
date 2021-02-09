//! @ The first task is to move the list from |head| to |temp_head| and go
//! into the enclosing semantic level. We also append the \.{\\parfillskip}
//! glue to the end of the paragraph, removing a space (or other glue node) if
//! it was there, since spaces usually precede blank lines and instances of
//! `\.{\$\$}'. The |par_fill_skip| is preceded by an infinite penalty, so
//! it will never be considered as a potential breakpoint.
//!
//! This code assumes that a |glue_node| and a |penalty_node| occupy the
//! same number of |mem|~words.
//! @^data structure assumptions@>
//!
//! @<Get ready to start...@>=
//! link(temp_head):=link(head);
//! if is_char_node(tail) then tail_append(new_penalty(inf_penalty))
//! else if type(tail)<>glue_node then tail_append(new_penalty(inf_penalty))
//! else  begin type(tail):=penalty_node; delete_glue_ref(glue_ptr(tail));
//!   flush_node_list(leader_ptr(tail)); penalty(tail):=inf_penalty;
//!   end;
//! link(tail):=new_param_glue(par_fill_skip_code);
//! init_cur_lang:=prev_graf mod @'200000;
//! init_l_hyf:=prev_graf div @'20000000;
//! init_r_hyf:=(prev_graf div @'200000) mod @'100;
//! pop_nest;
//!
