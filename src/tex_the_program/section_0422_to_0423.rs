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
