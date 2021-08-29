//! @ @<Determine vertical glue shrink setting...@>=
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
//!   @<Report an overfull vbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//!   end
//! else if o=normal then if list_ptr(r)<>null then
//!   @<Report a tight vbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//! return;
//! end
//!
//! @ @<Report an overfull vbox and |goto common_ending|, if...@>=
//! if (-x-total_shrink[normal]>vfuzz)or(vbadness<100) then
//!   begin print_ln; print_nl("Overfull \vbox (");
//! @.Overfull \\vbox...@>
//!   print_scaled(-x-total_shrink[normal]); print("pt too high");
//!   goto common_ending;
//!   end
//!
//! @ @<Report a tight vbox and |goto common_ending|, if...@>=
//! begin last_badness:=badness(-x,total_shrink[normal]);
//! if last_badness>vbadness then
//!   begin print_ln; print_nl("Tight \vbox (badness "); print_int(last_badness);
//! @.Tight \\vbox...@>
//!   goto common_ending;
//!   end;
//! end
//!
