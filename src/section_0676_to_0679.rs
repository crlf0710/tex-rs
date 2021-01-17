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
//! @ When a box is being appended to the current vertical list, the
//! baselineskip calculation is handled by the |append_to_vlist| routine.
//!
//! @p procedure append_to_vlist(@!b:pointer);
//! var d:scaled; {deficiency of space between baselines}
//! @!p:pointer; {a new glue node}
//! begin if prev_depth>ignore_depth then
//!   begin d:=width(baseline_skip)-prev_depth-height(b);
//!   if d<line_skip_limit then p:=new_param_glue(line_skip_code)
//!   else  begin p:=new_skip_param(baseline_skip_code);
//!     width(temp_ptr):=d; {|temp_ptr=glue_ptr(p)|}
//!     end;
//!   link(tail):=p; tail:=p;
//!   end;
//! link(tail):=b; tail:=b; prev_depth:=depth(b);
//! end;
//!
