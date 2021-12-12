//! @ Here we save memory space in a common case.
//!
//! @<Simplify a trivial box@>=
//! q:=list_ptr(x);
//! if is_char_node(q) then
//!   begin r:=link(q);
//!   if r<>null then if link(r)=null then if not is_char_node(r) then
//!    if type(r)=kern_node then {unneeded italic correction}
//!     begin free_node(r,small_node_size); link(q):=null;
//!     end;
//!   end
//!
