//! @ Shrinkability is never infinite in a paragraph;
//! we can shrink the line from |r| to |cur_p| by at most |cur_active_width[6]|.
//
// @<Set the value of |b| to the badness for shrinking...@>=
pub(crate) macro Set_the_value_of_b_to_the_badness_for_shrinking_the_line__and_compute_the_corresponding_fit_class($globals:expr, $b:expr, $fit_class:expr, $shortfall:expr) {{
    // begin if -shortfall>cur_active_width[6] then b:=inf_bad+1
    if -$shortfall > $globals.cur_active_width[6] {
        $b = inf_bad + 1;
    }
    // else b:=badness(-shortfall,cur_active_width[6]);
    else {
        $b = badness($globals, -$shortfall, $globals.cur_active_width[6]);
    }
    // if b>12 then fit_class:=tight_fit@+else fit_class:=decent_fit;
    if $b > 12 {
        $fit_class = fit_class_kind::tight_fit;
    } else {
        $fit_class = fit_class_kind::decent_fit;
    }
    // end
    use crate::section_0108::badness;
    use crate::section_0108::inf_bad;
    use crate::section_0817::fit_class_kind;
}}
