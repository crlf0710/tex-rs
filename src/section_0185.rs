//! @ @<Display special fields of the unset node |p|@>=
//! begin if span_count(p)<>min_quarterword then
//!   begin print(" ("); print_int(qo(span_count(p))+1);
//!   print(" columns)");
//!   end;
//! if glue_stretch(p)<>0 then
//!   begin print(", stretch "); print_glue(glue_stretch(p),glue_order(p),0);
//!   end;
//! if glue_shrink(p)<>0 then
//!   begin print(", shrink "); print_glue(glue_shrink(p),glue_sign(p),0);
//!   end;
//! end
//!
