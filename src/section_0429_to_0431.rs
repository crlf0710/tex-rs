//! @ When a |glue_val| changes to a |dimen_val|, we use the width component
//! of the glue; there is no need to decrease the reference count, since it
//! has not yet been increased.  When a |dimen_val| changes to an |int_val|,
//! we use scaled points so that the value doesn't actually change. And when a
//! |mu_val| changes to a |glue_val|, the value doesn't change either.
//!
//! @<Convert \(c)|cur_val| to a lower level@>=
//! begin if cur_val_level=glue_val then cur_val:=width(cur_val)
//! else if cur_val_level=mu_val then mu_error;
//! decr(cur_val_level);
//! end
//!
//! @ If |cur_val| points to a glue specification at this point, the reference
//! count for the glue does not yet include the reference by |cur_val|.
//! If |negative| is |true|, |cur_val_level| is known to be |<=mu_val|.
//!
//! @<Fix the reference count, if any, ...@>=
//! if negative then
//!   if cur_val_level>=glue_val then
//!     begin cur_val:=new_spec(cur_val);
//!     @<Negate all three glue components of |cur_val|@>;
//!     end
//!   else negate(cur_val)
//! else if (cur_val_level>=glue_val)and(cur_val_level<=mu_val) then
//!   add_glue_ref(cur_val)
//!
//! @ @<Negate all three...@>=
//! begin negate(width(cur_val));
//! negate(stretch(cur_val));
//! negate(shrink(cur_val));
//! end
//!
