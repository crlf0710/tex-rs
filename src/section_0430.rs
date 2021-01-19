//! @ If |cur_val| points to a glue specification at this point, the reference
//! count for the glue does not yet include the reference by |cur_val|.
//! If |negative| is |true|, |cur_val_level| is known to be |<=mu_val|.
//
// @<Fix the reference count, if any, ...@>=
macro_rules! Fix_the_reference_count__if_any__and_negate_cur_val_if_negative {
    ($globals:expr, $negative:expr) => {{
        // if negative then
        if $negative {
            // if cur_val_level>=glue_val then
            if $globals.cur_val_level >= cur_val_level_kind::glue_val {
                // begin cur_val:=new_spec(cur_val);
                $globals.cur_val = new_spec($globals, $globals.cur_val as pointer)? as _;
                // @<Negate all three glue components of |cur_val|@>;
                todo!("Negate");
                // end
            }
            // else negate(cur_val)
            else {
                negate!($globals.cur_val);
            }
        }
        // else if (cur_val_level>=glue_val)and(cur_val_level<=mu_val) then
        else if $globals.cur_val_level >= cur_val_level_kind::glue_val && $globals.cur_val_level <= cur_val_level_kind::mu_val {
            // add_glue_ref(cur_val)
            add_glue_ref!($globals, $globals.cur_val as pointer);
        }
        use crate::section_0151::new_spec;
    }}
}
