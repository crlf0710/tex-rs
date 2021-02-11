//! @ @<Change discretionary to compulsory...@>=
//! begin t:=replace_count(q);
//! @<Destroy the |t| nodes following |q|, and
//!    make |r| point to the following node@>;
//! if post_break(q)<>null then @<Transplant the post-break list@>;
//! if pre_break(q)<>null then @<Transplant the pre-break list@>;
//! link(q):=r; disc_break:=true;
//! end
//!
//! @ @<Destroy the |t| nodes following |q|...@>=
//! if t=0 then r:=link(q)
//! else  begin r:=q;
//!   while t>1 do
//!     begin r:=link(r); decr(t);
//!     end;
//!   s:=link(r);
//!   r:=link(s); link(s):=null;
//!   flush_node_list(link(q)); replace_count(q):=0;
//!   end
//!
//! @ We move the post-break list from inside node |q| to the main list by
//! re\-attaching it just before the present node |r|, then resetting |r|.
//!
//! @<Transplant the post-break list@>=
//! begin s:=post_break(q);
//! while link(s)<>null do s:=link(s);
//! link(s):=r; r:=post_break(q); post_break(q):=null; post_disc_break:=true;
//! end
//!
//! @ We move the pre-break list from inside node |q| to the main list by
//! re\-attaching it just after the present node |q|, then resetting |q|.
//!
//! @<Transplant the pre-break list@>=
//! begin s:=pre_break(q); link(q):=s;
//! while link(s)<>null do s:=link(s);
//! pre_break(q):=null; q:=s;
//! end
//!
