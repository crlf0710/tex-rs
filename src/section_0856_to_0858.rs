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
