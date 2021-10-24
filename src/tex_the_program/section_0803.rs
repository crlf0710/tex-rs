//! @ Merging of two span-node lists is a typical exercise in the manipulation of
//! linearly linked data structures. The essential invariant in the following
//! |repeat| loop is that we want to dispense with node |r|, in |q|'s list,
//! and |u| is its successor; all nodes of |p|'s list up to and including |s|
//! have been processed, and the successor of |s| matches |r| or precedes |r|
//! or follows |r|, according as |link(r)=n| or |link(r)>n| or |link(r)<n|.
//!
//! @<Merge the widths...@>=
//! begin t:=width(q)+width(glue_ptr(link(q)));
//! r:=info(q); s:=end_span; info(s):=p; n:=min_quarterword+1;
//! repeat width(r):=width(r)-t; u:=info(r);
//! while link(r)>n do
//!   begin s:=info(s); n:=link(info(s))+1;
//!   end;
//! if link(r)<n then
//!   begin info(r):=info(s); info(s):=r; decr(link(r)); s:=r;
//!   end
//! else  begin if width(r)>width(info(s)) then width(info(s)):=width(r);
//!   free_node(r,span_node_size);
//!   end;
//! r:=u;
//! until r=end_span;
//! end
//!
