//! @ When the debugging routine |search_mem| is looking for pointers having a
//! given value, it is interested only in regions 1 to~3 of~|eqtb|, and in the
//! first part of region~4.
//!
//! @<Search |eqtb| for equivalents equal to |p|@>=
//! for q:=active_base to box_base+255 do
//!   begin if equiv(q)=p then
//!     begin print_nl("EQUIV("); print_int(q); print_char(")");
//!     end;
//!   end
//!
