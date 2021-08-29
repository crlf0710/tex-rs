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
