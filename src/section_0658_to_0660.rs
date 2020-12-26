//! @ @<Determine horizontal glue stretch setting...@>=
//! begin @<Determine the stretch order@>;
//! glue_order(r):=o; glue_sign(r):=stretching;
//! if total_stretch[o]<>0 then glue_set(r):=unfloat(x/total_stretch[o])
//! @^real division@>
//! else  begin glue_sign(r):=normal;
//!   set_glue_ratio_zero(glue_set(r)); {there's nothing to stretch}
//!   end;
//! if o=normal then if list_ptr(r)<>null then
//!   @<Report an underfull hbox and |goto common_ending|, if this box
//!     is sufficiently bad@>;
//! return;
//! end
//!
//! @ @<Determine the stretch order@>=
//! if total_stretch[filll]<>0 then o:=filll
//! else if total_stretch[fill]<>0 then o:=fill
//! else if total_stretch[fil]<>0 then o:=fil
//! else o:=normal
//!
//! @ @<Report an underfull hbox and |goto common_ending|, if...@>=
//! begin last_badness:=badness(x,total_stretch[normal]);
//! if last_badness>hbadness then
//!   begin print_ln;
//!   if last_badness>100 then print_nl("Underfull")@+else print_nl("Loose");
//!   print(" \hbox (badness "); print_int(last_badness);
//! @.Underfull \\hbox...@>
//! @.Loose \\hbox...@>
//!   goto common_ending;
//!   end;
//! end
//!
