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
