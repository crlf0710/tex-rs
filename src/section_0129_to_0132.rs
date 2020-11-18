//! @ Here we delete node |p| from the ring, and let |rover| rove around.
//!
//! @<Allocate entire...@>=
//! begin rover:=rlink(p); t:=llink(p);
//! llink(rover):=t; rlink(t):=rover;
//! goto found;
//! end
//!
//! @ Conversely, when some variable-size node |p| of size |s| is no longer needed,
//! the operation |free_node(p,s)| will make its words available, by inserting
//! |p| as a new empty node just before where |rover| now points.
//! @^inner loop@>
//!
//! @p procedure free_node(@!p:pointer; @!s:halfword); {variable-size node
//!   liberation}
//! var q:pointer; {|llink(rover)|}
//! begin node_size(p):=s; link(p):=empty_flag;
//! q:=llink(rover); llink(p):=q; rlink(p):=rover; {set both links}
//! llink(rover):=p; rlink(q):=p; {insert |p| into the ring}
//! @!stat var_used:=var_used-s;@+tats@;{maintain statistics}
//! end;
//!
//! @ Just before \.{INITEX} writes out the memory, it sorts the doubly linked
//! available space list. The list is probably very short at such times, so a
//! simple insertion sort is used. The smallest available location will be
//! pointed to by |rover|, the next-smallest by |rlink(rover)|, etc.
//!
//! @p @!init procedure sort_avail; {sorts the available variable-size nodes
//!   by location}
//! var p,@!q,@!r: pointer; {indices into |mem|}
//! @!old_rover:pointer; {initial |rover| setting}
//! begin p:=get_node(@'10000000000); {merge adjacent free areas}
//! p:=rlink(rover); rlink(rover):=max_halfword; old_rover:=rover;
//! while p<>old_rover do @<Sort \(p)|p| into the list starting at |rover|
//!   and advance |p| to |rlink(p)|@>;
//! p:=rover;
//! while rlink(p)<>max_halfword do
//!   begin llink(rlink(p)):=p; p:=rlink(p);
//!   end;
//! rlink(p):=rover; llink(rover):=p;
//! end;
//! tini
//!
//! @ The following |while| loop is guaranteed to
//! terminate, since the list that starts at
//! |rover| ends with |max_halfword| during the sorting procedure.
//!
//! @<Sort \(p)|p|...@>=
//! if p<rover then
//!   begin q:=p; p:=rlink(q); rlink(q):=rover; rover:=q;
//!   end
//! else  begin q:=rover;
//!   while rlink(q)<p do q:=rlink(q);
//!   r:=rlink(p); rlink(p):=rlink(q); rlink(q):=p; p:=r;
//!   end
//!
