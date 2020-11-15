//! @ There's also a |fast_get_avail| routine, which saves the procedure-call
//! overhead at the expense of extra programming. This routine is used in
//! the places that would otherwise account for the most calls of |get_avail|.
//! @^inner loop@>
//!
//! @d fast_get_avail(#)==@t@>@;@/
//!   begin #:=avail; {avoid |get_avail| if possible, to save time}
//!   if #=null then #:=get_avail
//!   else  begin avail:=link(#); link(#):=null;
//!     @!stat incr(dyn_used);@+tats@/
//!     end;
//!   end
//!