//! @ The first part of the following code is part of \TeX's inner loop, so
//! we don't want to waste any time. The current active node, namely node |r|,
//! contains the line number that will be considered next. At the end of the
//! list we have arranged the data structure so that |r=last_active| and
//! |line_number(last_active)>old_l|.
//! @^inner loop@>
//
// @<If a line number class...@>=
macro_rules! If_a_line_number_class_has_ended__create_new_active_nodes_for_the_best_feasible_breaks_in_that_class__then_return_if_r_eq_last_active__otherwise_compute_the_new_line_width {
    ($globals:expr, $r:expr, $prev_r:expr, $prev_prev_r:expr, $l:expr, $old_l:expr, $line_width:expr, $break_type:expr, $no_break_yet:expr) => {{
        trace_span!("If a line number class...");
        // begin l:=line_number(r);
        $l = line_number!($globals, $r);
        // if l>old_l then
        if $l > $old_l {
            // begin {now we are no longer in the inner loop}
            /// now we are no longer in the inner loop
            const _: () = ();
            // if (minimum_demerits<awful_bad)and@|
            //     ((old_l<>easy_line)or(r=last_active)) then
            if $globals.minimum_demerits < awful_bad
                && ($old_l != $globals.easy_line || $r == last_active!())
            {
                // @<Create new active nodes for the best feasible breaks
                //   just found@>;
                Create_new_active_nodes_for_the_best_feasible_breaks_just_found!(
                    $globals,
                    $r,
                    $prev_r,
                    $prev_prev_r,
                    $break_type,
                    $no_break_yet
                );
            }
            // if r=last_active then return;
            if $r == last_active!() {
                return_nojump!();
            }
            // @<Compute the new line width@>;
            Compute_the_new_line_width!($globals, $l, $old_l, $line_width);
            // end;
        }
        // end
        use crate::section_0833::awful_bad;
    }};
}
