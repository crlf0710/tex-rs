//! @ @<Compute result of |multiply| or |divide|...@>=
//! begin scan_int;
//! if p<glue_val then
//!   if q=multiply then
//!     if p=int_val then cur_val:=mult_integers(eqtb[l].int,cur_val)
//!     else cur_val:=nx_plus_y(eqtb[l].int,cur_val,0)
//!   else cur_val:=x_over_n(eqtb[l].int,cur_val)
//! else  begin s:=equiv(l); r:=new_spec(s);
//!   if q=multiply then
//!     begin width(r):=nx_plus_y(width(s),cur_val,0);
//!     stretch(r):=nx_plus_y(stretch(s),cur_val,0);
//!     shrink(r):=nx_plus_y(shrink(s),cur_val,0);
//!     end
//!   else  begin width(r):=x_over_n(width(s),cur_val);
//!     stretch(r):=x_over_n(stretch(s),cur_val);
//!     shrink(r):=x_over_n(shrink(s),cur_val);
//!     end;
//!   cur_val:=r;
//!   end;
//! end
//!
