//! @ @<Append an insertion to the current page and |goto contribute|@>=
//! begin if page_contents=empty then freeze_page_specs(inserts_only);
//! n:=subtype(p); r:=page_ins_head;
//! while n>=subtype(link(r)) do r:=link(r);
//! n:=qo(n);
//! if subtype(r)<>qi(n) then
//!   @<Create a page insertion node with |subtype(r)=qi(n)|, and
//!     include the glue correction for box |n| in the
//!     current page state@>;
//! if type(r)=split_up then insert_penalties:=insert_penalties+float_cost(p)
//! else  begin last_ins_ptr(r):=p;
//!   delta:=page_goal-page_total-page_depth+page_shrink;
//!     {this much room is left if we shrink the maximum}
//!   if count(n)=1000 then h:=height(p)
//!   else h:=x_over_n(height(p),1000)*count(n); {this much room is needed}
//!   if ((h<=0)or(h<=delta))and(height(p)+height(r)<=dimen(n)) then
//!     begin page_goal:=page_goal-h; height(r):=height(r)+height(p);
//!     end
//!   else @<Find the best way to split the insertion, and change
//!     |type(r)| to |split_up|@>;
//!   end;
//! goto contribute;
//! end
//!
//! @ We take note of the value of \.{\\skip} |n| and the height plus depth
//! of \.{\\box}~|n| only when the first \.{\\insert}~|n| node is
//! encountered for a new page. A user who changes the contents of \.{\\box}~|n|
//! after that first \.{\\insert}~|n| had better be either extremely careful
//! or extremely lucky, or both.
//!
//! @<Create a page insertion node...@>=
//! begin q:=get_node(page_ins_node_size); link(q):=link(r); link(r):=q; r:=q;
//! subtype(r):=qi(n); type(r):=inserting; ensure_vbox(n);
//! if box(n)=null then height(r):=0
//! else height(r):=height(box(n))+depth(box(n));
//! best_ins_ptr(r):=null;@/
//! q:=skip(n);
//! if count(n)=1000 then h:=height(r)
//! else h:=x_over_n(height(r),1000)*count(n);
//! page_goal:=page_goal-h-width(q);@/
//! page_so_far[2+stretch_order(q)]:=@|page_so_far[2+stretch_order(q)]+stretch(q);@/
//! page_shrink:=page_shrink+shrink(q);
//! if (shrink_order(q)<>normal)and(shrink(q)<>0) then
//!   begin print_err("Infinite glue shrinkage inserted from "); print_esc("skip");
//! @.Infinite glue shrinkage...@>
//!   print_int(n);
//!   help3("The correction glue for page breaking with insertions")@/
//!     ("must have finite shrinkability. But you may proceed,")@/
//!     ("since the offensive shrinkability has been made finite.");
//!   error;
//!   end;
//! end
//!
//! @ Here is the code that will split a long footnote between pages, in an
//! emergency. The current situation deserves to be recapitulated: Node |p|
//! is an insertion into box |n|; the insertion will not fit, in its entirety,
//! either because it would make the total contents of box |n| greater than
//! \.{\\dimen} |n|, or because it would make the incremental amount of growth
//! |h| greater than the available space |delta|, or both. (This amount |h| has
//! been weighted by the insertion scaling factor, i.e., by \.{\\count} |n|
//! over 1000.) Now we will choose the best way to break the vlist of the
//! insertion, using the same criteria as in the \.{\\vsplit} operation.
//!
//! @<Find the best way to split the insertion...@>=
//! begin if count(n)<=0 then w:=max_dimen
//! else  begin w:=page_goal-page_total-page_depth;
//!   if count(n)<>1000 then w:=x_over_n(w,count(n))*1000;
//!   end;
//! if w>dimen(n)-height(r) then w:=dimen(n)-height(r);
//! q:=vert_break(ins_ptr(p),w,depth(p));
//! height(r):=height(r)+best_height_plus_depth;
//! @!stat if tracing_pages>0 then @<Display the insertion split cost@>;@+tats@;@/
//! if count(n)<>1000 then
//!   best_height_plus_depth:=x_over_n(best_height_plus_depth,1000)*count(n);
//! page_goal:=page_goal-best_height_plus_depth;
//! type(r):=split_up; broken_ptr(r):=q; broken_ins(r):=p;
//! if q=null then insert_penalties:=insert_penalties+eject_penalty
//! else if type(q)=penalty_node then insert_penalties:=insert_penalties+penalty(q);
//! end
//!
//! @ @<Display the insertion split cost@>=
//! begin begin_diagnostic; print_nl("% split"); print_int(n);
//! @.split@>
//! print(" to "); print_scaled(w);
//! print_char(","); print_scaled(best_height_plus_depth);@/
//! print(" p=");
//! if q=null then print_int(eject_penalty)
//! else if type(q)=penalty_node then print_int(penalty(q))
//! else print_char("0");
//! end_diagnostic(false);
//! end
//!
