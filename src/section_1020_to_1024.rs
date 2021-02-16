//! @ We will set |best_ins_ptr:=null| and package the box corresponding to
//! insertion node~|r|, just after making the final insertion into that box.
//! If this final insertion is `|split_up|', the remainder after splitting
//! and pruning (if any) will be carried over to the next page.
//!
//! @<Either insert the material specified by node |p| into...@>=
//! begin r:=link(page_ins_head);
//! while subtype(r)<>subtype(p) do r:=link(r);
//! if best_ins_ptr(r)=null then wait:=true
//! else  begin wait:=false; s:=last_ins_ptr(r); link(s):=ins_ptr(p);
//!   if best_ins_ptr(r)=p then
//!     @<Wrap up the box specified by node |r|, splitting node |p| if
//!     called for; set |wait:=true| if node |p| holds a remainder after
//!     splitting@>
//!   else  begin while link(s)<>null do s:=link(s);
//!     last_ins_ptr(r):=s;
//!     end;
//!   end;
//! @<Either append the insertion node |p| after node |q|, and remove it
//!   from the current page, or delete |node(p)|@>;
//! end
//!
//! @ @<Wrap up the box specified by node |r|, splitting node |p| if...@>=
//! begin if type(r)=split_up then
//!   if (broken_ins(r)=p)and(broken_ptr(r)<>null) then
//!     begin while link(s)<>broken_ptr(r) do s:=link(s);
//!     link(s):=null;
//!     split_top_skip:=split_top_ptr(p);
//!     ins_ptr(p):=prune_page_top(broken_ptr(r));
//!     if ins_ptr(p)<>null then
//!       begin temp_ptr:=vpack(ins_ptr(p),natural);
//!       height(p):=height(temp_ptr)+depth(temp_ptr);
//!       free_node(temp_ptr,box_node_size); wait:=true;
//!       end;
//!     end;
//! best_ins_ptr(r):=null;
//! n:=qo(subtype(r));
//! temp_ptr:=list_ptr(box(n));
//! free_node(box(n),box_node_size);
//! box(n):=vpack(temp_ptr,natural);
//! end
//!
//! @ @<Either append the insertion node |p|...@>=
//! link(prev_p):=link(p); link(p):=null;
//! if wait then
//!   begin link(q):=p; q:=p; incr(insert_penalties);
//!   end
//! else  begin delete_glue_ref(split_top_ptr(p));
//!   free_node(p,ins_node_size);
//!   end;
//! p:=prev_p
//!
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
