//! @ When a line must stretch, the available stretchability can be found in the
//! subarray |cur_active_width[2..5]|, in units of points, fil, fill, and filll.
//!
//! The present section is part of \TeX's inner loop, and it is most often performed
//! when the badness is infinite; therefore it is worth while to make a quick
//! test for large width excess and small stretchability, before calling the
//! |badness| subroutine.
//! @^inner loop@>
//
// @<Set the value of |b| to the badness for stretching...@>=
macro_rules! Set_the_value_of_b_to_the_badness_for_stretching_the_line__and_compute_the_corresponding_fit_class {
    ($globals:expr, $b:expr, $fit_class:expr, $shortfall:expr) => {{
        // if (cur_active_width[3]<>0)or(cur_active_width[4]<>0)or@|
        //   (cur_active_width[5]<>0) then
        if $globals.cur_active_width[3] != scaled::zero() || $globals.cur_active_width[4] != scaled::zero() ||
            $globals.cur_active_width[5] != scaled::zero() { 
            // begin b:=0; fit_class:=decent_fit; {infinite stretch}
            $b = 0;
            $fit_class = fit_class_kind::decent_fit;
            /// infinite stretch
            const _ : () = ();
            // end
        }
        // else  begin if shortfall>7230584 then if cur_active_width[2]<1663497 then
        else {
            region_forward_label!(
            |'done1|
            {
            if $shortfall > scaled::new_from_inner(7230584) && $globals.cur_active_width[2] < scaled::new_from_inner(1663497) {
                // begin b:=inf_bad; fit_class:=very_loose_fit; goto done1;
                $b = inf_bad;
                $fit_class = fit_class_kind::very_loose_fit;
                goto_forward_label!('done1);
                // end;
            }
            // b:=badness(shortfall,cur_active_width[2]);
            $b = badness($globals, $shortfall, $globals.cur_active_width[2]);
            // if b>12 then
            if $b > 12 {
                // if b>99 then fit_class:=very_loose_fit
                if $b > 99 {
                    $fit_class = fit_class_kind::very_loose_fit;
                }
                // else fit_class:=loose_fit
                else {
                    $fit_class = fit_class_kind::loose_fit;
                }
            }
            // else fit_class:=decent_fit;
            else {
                $fit_class = fit_class_kind::decent_fit;
            }
            // done1:
            }
            'done1 <-
            );
            // end
        }
        use crate::section_0108::badness;
    }}
}