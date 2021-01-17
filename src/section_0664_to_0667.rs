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
//! @ @<Determine the shrink order@>=
//! if total_shrink[filll]<>0 then o:=filll
//! else if total_shrink[fill]<>0 then o:=fill
//! else if total_shrink[fil]<>0 then o:=fil
//! else o:=normal
//!
//! @ @<Report an overfull hbox and |goto common_ending|, if...@>=
//! if (-x-total_shrink[normal]>hfuzz)or(hbadness<100) then
//!   begin if (overfull_rule>0)and(-x-total_shrink[normal]>hfuzz) then
//!     begin while link(q)<>null do q:=link(q);
//!     link(q):=new_rule;
//!     width(link(q)):=overfull_rule;
//!     end;
//!   print_ln; print_nl("Overfull \hbox (");
//! @.Overfull \\hbox...@>
//!   print_scaled(-x-total_shrink[normal]); print("pt too wide");
//!   goto common_ending;
//!   end
//!
//! @ @<Report a tight hbox and |goto common_ending|, if...@>=
//! begin last_badness:=badness(-x,total_shrink[normal]);
//! if last_badness>hbadness then
//!   begin print_ln; print_nl("Tight \hbox (badness "); print_int(last_badness);
//! @.Tight \\hbox...@>
//!   goto common_ending;
//!   end;
//! end
//!
