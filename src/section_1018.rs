//! @ If many insertions are supposed to go into the same box, we want to know
//! the position of the last node in that box, so that we don't need to waste time
//! when linking further information into it. The |last_ins_ptr| fields of the
//! page insertion nodes are therefore used for this purpose during the
//! packaging phase.
//!
//! @<Prepare all the boxes involved in insertions to act as queues@>=
//! begin r:=link(page_ins_head);
//! while r<>page_ins_head do
//!   begin if best_ins_ptr(r)<>null then
//!     begin n:=qo(subtype(r)); ensure_vbox(n);
//!     if box(n)=null then box(n):=new_null_box;
//!     p:=box(n)+list_offset;
//!     while link(p)<>null do p:=link(p);
//!     last_ins_ptr(r):=p;
//!     end;
//!   r:=link(r);
//!   end;
//! end
//!
