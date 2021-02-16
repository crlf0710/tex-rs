//! @ When the following code is executed, the current page runs from node
//! |link(page_head)| to node |prev_p|, and the nodes from |p| to |page_tail|
//! are to be placed back at the front of the contribution list. Furthermore
//! the heldover insertions appear in a list from |link(hold_head)| to |q|; we
//! will put them into the current page list for safekeeping while the user's
//! output routine is active.  We might have |q=hold_head|; and |p=null| if
//! and only if |prev_p=page_tail|. Error messages are suppressed within
//! |vpackage|, since the box might appear to be overfull or underfull simply
//! because the stretch and shrink from the \.{\\skip} registers for inserts
//! are not actually present in the box.
//!
//! @<Break the current page at node |p|, put it...@>=
//! if p<>null then
//!   begin if link(contrib_head)=null then
//!     if nest_ptr=0 then tail:=page_tail
//!     else contrib_tail:=page_tail;
//!   link(page_tail):=link(contrib_head);
//!   link(contrib_head):=p;
//!   link(prev_p):=null;
//!   end;
//! save_vbadness:=vbadness; vbadness:=inf_bad;
//! save_vfuzz:=vfuzz; vfuzz:=max_dimen; {inhibit error messages}
//! box(255):=vpackage(link(page_head),best_size,exactly,page_max_depth);
//! vbadness:=save_vbadness; vfuzz:=save_vfuzz;
//! if last_glue<>max_halfword then delete_glue_ref(last_glue);
//! @<Start a new current page@>; {this sets |last_glue:=max_halfword|}
//! if q<>hold_head then
//!   begin link(page_head):=link(hold_head); page_tail:=q;
//!   end
//!
