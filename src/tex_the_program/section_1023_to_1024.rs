//! @ The list of heldover insertions, running from |link(page_head)| to
//! |page_tail|, must be moved to the contribution list when the user has
//! specified no output routine.
//!
//! @<Perform the default output routine@>=
//! begin if link(page_head)<>null then
//!   begin if link(contrib_head)=null then
//!     if nest_ptr=0 then tail:=page_tail@+else contrib_tail:=page_tail
//!   else link(page_tail):=link(contrib_head);
//!   link(contrib_head):=link(page_head);
//!   link(page_head):=null; page_tail:=page_head;
//!   end;
//! ship_out(box(255)); box(255):=null;
//! end
//!
//! @ @<Explain that too many dead cycles have occurred in a row@>=
//! begin print_err("Output loop---"); print_int(dead_cycles);
//! @.Output loop...@>
//! print(" consecutive dead cycles");
//! help3("I've concluded that your \output is awry; it never does a")@/
//! ("\shipout, so I'm shipping \box255 out myself. Next time")@/
//! ("increase \maxdeadcycles if you want me to be more patient!"); error;
//! end
//!
