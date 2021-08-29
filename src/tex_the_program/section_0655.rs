//! @ Although node |q| is not necessarily the immediate predecessor of node |p|,
//! it always points to some node in the list preceding |p|. Thus, we can delete
//! nodes by moving |q| when necessary. The algorithm takes linear time, and the
//! extra computation does not intrude on the inner loop unless it is necessary
//! to make a deletion.
//! @^inner loop@>
//!
//! @<Transfer node |p| to the adjustment list@>=
//! begin while link(q)<>p do q:=link(q);
//! if type(p)=adjust_node then
//!   begin link(adjust_tail):=adjust_ptr(p);
//!   while link(adjust_tail)<>null do adjust_tail:=link(adjust_tail);
//!   p:=link(p); free_node(link(q),small_node_size);
//!   end
//! else  begin link(adjust_tail):=p; adjust_tail:=p; p:=link(p);
//!   end;
//! link(q):=p; p:=q;
//! end
//!
