//! @ When a line must stretch, the available stretchability can be found in the
//! subarray |cur_active_width[2..5]|, in units of points, fil, fill, and filll.
//!
//! The present section is part of \TeX's inner loop, and it is most often performed
//! when the badness is infinite; therefore it is worth while to make a quick
//! test for large width excess and small stretchability, before calling the
//! |badness| subroutine.
//! @^inner loop@>
//!
//! @<Set the value of |b| to the badness for stretching...@>=
//! if (cur_active_width[3]<>0)or(cur_active_width[4]<>0)or@|
//!   (cur_active_width[5]<>0) then
//!   begin b:=0; fit_class:=decent_fit; {infinite stretch}
//!   end
//! else  begin if shortfall>7230584 then if cur_active_width[2]<1663497 then
//!     begin b:=inf_bad; fit_class:=very_loose_fit; goto done1;
//!     end;
//!   b:=badness(shortfall,cur_active_width[2]);
//!   if b>12 then
//!     if b>99 then fit_class:=very_loose_fit
//!     else fit_class:=loose_fit
//!   else fit_class:=decent_fit;
//!   done1:
//!   end
//!
//! @ Shrinkability is never infinite in a paragraph;
//! we can shrink the line from |r| to |cur_p| by at most |cur_active_width[6]|.
//!
//! @<Set the value of |b| to the badness for shrinking...@>=
//! begin if -shortfall>cur_active_width[6] then b:=inf_bad+1
//! else b:=badness(-shortfall,cur_active_width[6]);
//! if b>12 then fit_class:=tight_fit@+else fit_class:=decent_fit;
//! end
//!
//! @ During the final pass, we dare not lose all active nodes, lest we lose
//! touch with the line breaks already found. The code shown here makes sure
//! that such a catastrophe does not happen, by permitting overfull boxes as
//! a last resort. This particular part of \TeX\ was a source of several subtle
//! bugs before the correct program logic was finally discovered; readers
//! who seek to ``improve'' \TeX\ should therefore think thrice before daring
//! to make any changes here.
//! @^overfull boxes@>
//!
//! @<Prepare to deactivate node~|r|, and |goto deactivate| unless...@>=
//! begin if final_pass and (minimum_demerits=awful_bad) and@|
//!    (link(r)=last_active) and
//!    (prev_r=active) then
//!   artificial_demerits:=true {set demerits zero, this break is forced}
//! else if b>threshold then goto deactivate;
//! node_r_stays_active:=false;
//! end
//!
//! @ When we get to this part of the code, the line from |r| to |cur_p| is
//! feasible, its badness is~|b|, and its fitness classification is |fit_class|.
//! We don't want to make an active node for this break yet, but we will
//! compute the total demerits and record them in the |minimal_demerits| array,
//! if such a break is the current champion among all ways to get to |cur_p|
//! in a given line-number class and fitness class.
//!
//! @<Record a new feasible break@>=
//! if artificial_demerits then d:=0
//! else @<Compute the demerits, |d|, from |r| to |cur_p|@>;
//! @!stat if tracing_paragraphs>0 then
//!   @<Print a symbolic description of this feasible break@>;
//! tats@;@/
//! d:=d+total_demerits(r); {this is the minimum total demerits
//!   from the beginning to |cur_p| via |r|}
//! if d<=minimal_demerits[fit_class] then
//!   begin minimal_demerits[fit_class]:=d;
//!   best_place[fit_class]:=break_node(r); best_pl_line[fit_class]:=l;
//!   if d<minimum_demerits then minimum_demerits:=d;
//!   end
//!
//! @ @<Print a symbolic description of this feasible break@>=
//! begin if printed_node<>cur_p then
//!   @<Print the list between |printed_node| and |cur_p|,
//!     then set |printed_node:=cur_p|@>;
//! print_nl("@@");
//! @.\AT!@>
//! if cur_p=null then print_esc("par")
//! else if type(cur_p)<>glue_node then
//!   begin if type(cur_p)=penalty_node then print_esc("penalty")
//!   else if type(cur_p)=disc_node then print_esc("discretionary")
//!   else if type(cur_p)=kern_node then print_esc("kern")
//!   else print_esc("math");
//!   end;
//! print(" via @@@@");
//! if break_node(r)=null then print_char("0")
//! else print_int(serial(break_node(r)));
//! print(" b=");
//! if b>inf_bad then print_char("*")@+else print_int(b);
//! @.*\relax@>
//! print(" p="); print_int(pi); print(" d=");
//! if artificial_demerits then print_char("*")@+else print_int(d);
//! end
//!
//! @ @<Print the list between |printed_node| and |cur_p|...@>=
//! begin print_nl("");
//! if cur_p=null then short_display(link(printed_node))
//! else  begin save_link:=link(cur_p);
//!   link(cur_p):=null; print_nl(""); short_display(link(printed_node));
//!   link(cur_p):=save_link;
//!   end;
//! printed_node:=cur_p;
//! end
//!
//! @ When the data for a discretionary break is being displayed, we will have
//! printed the |pre_break| and |post_break| lists; we want to skip over the
//! third list, so that the discretionary data will not appear twice.  The
//! following code is performed at the very end of |try_break|.
//!
//! @<Update the value of |printed_node|...@>=
//! if cur_p=printed_node then if cur_p<>null then if type(cur_p)=disc_node then
//!   begin t:=replace_count(cur_p);
//!   while t>0 do
//!     begin decr(t); printed_node:=link(printed_node);
//!     end;
//!   end
//!
//! @ @<Compute the demerits, |d|, from |r| to |cur_p|@>=
//! begin d:=line_penalty+b;
//! if abs(d)>=10000 then d:=100000000@+else d:=d*d;
//! if pi<>0 then
//!   if pi>0 then d:=d+pi*pi
//!   else if pi>eject_penalty then d:=d-pi*pi;
//! if (break_type=hyphenated)and(type(r)=hyphenated) then
//!   if cur_p<>null then d:=d+double_hyphen_demerits
//!   else d:=d+final_hyphen_demerits;
//! if abs(fit_class-fitness(r))>1 then d:=d+adj_demerits;
//! end
//!
//! @ When an active node disappears, we must delete an adjacent delta node if the
//! active node was at the beginning or the end of the active list, or if it
//! was surrounded by delta nodes. We also must preserve the property that
//! |cur_active_width| represents the length of material from |link(prev_r)|
//! to~|cur_p|.
//!
//! @d combine_two_deltas(#)==@|mem[prev_r+#].sc:=mem[prev_r+#].sc+mem[r+#].sc
//! @d downdate_width(#)==@|cur_active_width[#]:=cur_active_width[#]-
//!   mem[prev_r+#].sc
//!
//! @<Deactivate node |r|@>=
//! link(prev_r):=link(r); free_node(r,active_node_size);
//! if prev_r=active then @<Update the active widths, since the first active
//!   node has been deleted@>
//! else if type(prev_r)=delta_node then
//!   begin r:=link(prev_r);
//!   if r=last_active then
//!     begin do_all_six(downdate_width);
//!     link(prev_prev_r):=last_active;
//!     free_node(prev_r,delta_node_size); prev_r:=prev_prev_r;
//!     end
//!   else if type(r)=delta_node then
//!     begin do_all_six(update_width);
//!     do_all_six(combine_two_deltas);
//!     link(prev_r):=link(r); free_node(r,delta_node_size);
//!     end;
//!   end
//!
//! @ The following code uses the fact that |type(last_active)<>delta_node|. If the
//! active list has just become empty, we do not need to update the
//! |active_width| array, since it will be initialized when an active
//! node is next inserted.
//!
//! @d update_active(#)==active_width[#]:=active_width[#]+mem[r+#].sc
//!
//! @<Update the active widths,...@>=
//! begin r:=link(active);
//! if type(r)=delta_node then
//!   begin do_all_six(update_active);
//!   do_all_six(copy_to_cur_active);
//!   link(active):=link(r); free_node(r,delta_node_size);
//!   end;
//! end
//!
