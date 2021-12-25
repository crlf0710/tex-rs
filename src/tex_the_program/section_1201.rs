//! @ The user can force the equation number to go on a separate line
//! by causing its width to be zero.
//!
//! @<Squeeze the equation as much as possible...@>=
//! begin if (e<>0)and((w-total_shrink[normal]+q<=z)or@|
//!    (total_shrink[fil]<>0)or(total_shrink[fill]<>0)or
//!    (total_shrink[filll]<>0)) then
//!   begin free_node(b,box_node_size);
//!   b:=hpack(p,z-q,exactly);
//!   end
//! else  begin e:=0;
//!   if w>z then
//!     begin free_node(b,box_node_size);
//!     b:=hpack(p,z,exactly);
//!     end;
//!   end;
//! w:=width(b);
//! end
//!
