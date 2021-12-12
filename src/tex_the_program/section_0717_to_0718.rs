//! @ The |math_kern| subroutine removes |mu_glue| from a kern node, given
//! the value of the math unit.
//!
//! @p procedure math_kern(@!p:pointer;@!m:scaled);
//! var @!n:integer; {integer part of |m|}
//! @!f:scaled; {fraction part of |m|}
//! begin if subtype(p)=mu_glue then
//!   begin n:=x_over_n(m,@'200000); f:=remainder;@/
//!   if f<0 then
//!     begin decr(n); f:=f+@'200000;
//!     end;
//!   width(p):=mu_mult(width(p)); subtype(p):=explicit;
//!   end;
//! end;
//!
//! @ Sometimes it is necessary to destroy an mlist. The following
//! subroutine empties the current list, assuming that |abs(mode)=mmode|.
//!
//! @p procedure flush_math;
//! begin flush_node_list(link(head)); flush_node_list(incompleat_noad);
//! link(head):=null; tail:=head; incompleat_noad:=null;
//! end;
//!
