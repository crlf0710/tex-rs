//! @ Inside an \.{\\output} routine, a user may wish to look at the page totals
//! that were present at the moment when output was triggered.
//!
//! @d max_dimen==@'7777777777 {$2^{30}-1$}
//!
//! @<Fetch something on the |page_so_far|@>=
//! begin if (page_contents=empty) and (not output_active) then
//!   if m=0 then cur_val:=max_dimen@+else cur_val:=0
//! else cur_val:=page_so_far[m];
//! cur_val_level:=dimen_val;
//! end
//!
//! @ @<Fetch the |prev_graf|@>=
//! if mode=0 then scanned_result(0)(int_val) {|prev_graf=0| within \.{\\write}}
//! else begin nest[nest_ptr]:=cur_list; p:=nest_ptr;
//!   while abs(nest[p].mode_field)<>vmode do decr(p);
//!   scanned_result(nest[p].pg_field)(int_val);
//!   end
//!
//! @ @<Fetch the |par_shape| size@>=
//! begin if par_shape_ptr=null then cur_val:=0
//! else cur_val:=info(par_shape_ptr);
//! cur_val_level:=int_val;
//! end
//!

