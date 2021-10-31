//! @ @<Compleat...@>=
//! begin math_type(denominator(incompleat_noad)):=sub_mlist;
//! info(denominator(incompleat_noad)):=link(head);
//! if p=null then q:=incompleat_noad
//! else  begin q:=info(numerator(incompleat_noad));
//!   if type(q)<>left_noad then confusion("right");
//! @:this can't happen right}{\quad right@>
//!   info(numerator(incompleat_noad)):=link(q);
//!   link(q):=incompleat_noad; link(incompleat_noad):=p;
//!   end;
//! end
//!
