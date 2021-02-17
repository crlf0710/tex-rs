//! @ A mild optimization of the output is performed by the |dvi_pop|
//! routine, which issues a |pop| unless it is possible to cancel a
//! `|push| |pop|' pair. The parameter to |dvi_pop| is the byte address
//! following the old |push| that matches the new |pop|.
//!
//! @p procedure dvi_pop(@!l:integer);
//! begin if (l=dvi_offset+dvi_ptr)and(dvi_ptr>0) then decr(dvi_ptr)
//! else dvi_out(pop);
//! end;
//!
