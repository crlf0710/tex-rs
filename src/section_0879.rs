//! @ Glue and penalty and kern and math nodes are deleted at the beginning of
//! a line, except in the anomalous case that the node to be deleted is actually
//! one of the chosen breakpoints. Otherwise
//! the pruning done here is designed to match
//! the lookahead computation in |try_break|, where the |break_width| values
//! are computed for non-discretionary breakpoints.
//!
//! @<Prune unwanted nodes at the beginning of the next line@>=
//! begin r:=temp_head;
//! loop@+  begin q:=link(r);
//!   if q=cur_break(cur_p) then goto done1;
//!     {|cur_break(cur_p)| is the next breakpoint}
//!   {now |q| cannot be |null|}
//!   if is_char_node(q) then goto done1;
//!   if non_discardable(q) then goto done1;
//!   if type(q)=kern_node then if subtype(q)<>explicit then goto done1;
//!   r:=q; {now |type(q)=glue_node|, |kern_node|, |math_node|, or |penalty_node|}
//!   end;
//! done1: if r<>temp_head then
//!   begin link(r):=null; flush_node_list(link(temp_head));
//!   link(temp_head):=q;
//!   end;
//! end
//!
