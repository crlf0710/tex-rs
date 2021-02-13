//! @ @<Copy the templates from node |cur_loop| into node |p|@>=
//! q:=hold_head; r:=u_part(cur_loop);
//! while r<>null do
//!   begin link(q):=get_avail; q:=link(q); info(q):=info(r); r:=link(r);
//!   end;
//! link(q):=null; u_part(p):=link(hold_head);
//! q:=hold_head; r:=v_part(cur_loop);
//! while r<>null do
//!   begin link(q):=get_avail; q:=link(q); info(q):=info(r); r:=link(r);
//!   end;
//! link(q):=null; v_part(p):=link(hold_head)
//!
