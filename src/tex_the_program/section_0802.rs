//! @ @<Nullify |width(q)| and the tabskip glue following this column@>=
//! begin width(q):=0; r:=link(q); s:=glue_ptr(r);
//! if s<>zero_glue then
//!   begin add_glue_ref(zero_glue); delete_glue_ref(s);
//!   glue_ptr(r):=zero_glue;
//!   end;
//! end
//!
