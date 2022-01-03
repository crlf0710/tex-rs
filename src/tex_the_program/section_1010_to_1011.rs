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
