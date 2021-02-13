//! @ @<Determine horizontal glue shrink setting...@>=
//! begin @<Determine the shrink order@>;
//! glue_order(r):=o; glue_sign(r):=shrinking;
//! if total_shrink[o]<>0 then glue_set(r):=unfloat((-x)/total_shrink[o])
//! @^real division@>
//! else  begin glue_sign(r):=normal;
//!   set_glue_ratio_zero(glue_set(r)); {there's nothing to shrink}
//!   end;
//! if (total_shrink[o]<-x)and(o=normal)and(list_ptr(r)<>null) then
//!   begin last_badness:=1000000;
//!   set_glue_ratio_one(glue_set(r)); {use the maximum shrinkage}
//!   @<Report an overfull hbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//!   end
//! else if o=normal then if list_ptr(r)<>null then
//!   @<Report a tight hbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//! return;
//! end
//!
