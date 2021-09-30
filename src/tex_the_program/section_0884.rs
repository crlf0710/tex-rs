//! @ We move the post-break list from inside node |q| to the main list by
//! re\-attaching it just before the present node |r|, then resetting |r|.
//!
//! @<Transplant the post-break list@>=
//! begin s:=post_break(q);
//! while link(s)<>null do s:=link(s);
//! link(s):=r; r:=post_break(q); post_break(q):=null; post_disc_break:=true;
//! end
//!
