//! @ It is not necessary to create new active nodes having |minimal_demerits|
//! greater than
//! |minimum_demerits+abs(adj_demerits)|, since such active nodes will never
//! be chosen in the final paragraph breaks. This observation allows us to
//! omit a substantial number of feasible breakpoints from further consideration.
//
// @<Create new active nodes...@>=
macro_rules! Create_new_active_nodes_for_the_best_feasible_breaks_just_found {
    ($globals:expr, $r:expr, $prev_r:expr, $prev_prev_r:expr, $break_type:expr, $no_break_yet:expr) => {{
        // begin if no_break_yet then @<Compute the values of |break_width|@>;
        if $no_break_yet {
            Compute_the_values_of_break_width!($globals, $break_type, $no_break_yet);
        }
        // @<Insert a delta node to prepare for breaks at |cur_p|@>;
        Insert_a_delta_node_to_prepare_for_breaks_at_cur_p!($globals, $r, $prev_r, $prev_prev_r);
        // if abs(adj_demerits)>=awful_bad-minimum_demerits then
        if adj_demerits!($globals).abs() >= awful_bad - $globals.minimum_demerits {
            // minimum_demerits:=awful_bad-1
            $globals.minimum_demerits = awful_bad - 1;
        }
        // else minimum_demerits:=minimum_demerits+abs(adj_demerits);
        else {
            $globals.minimum_demerits += adj_demerits!($globals).abs();
        }
        // for fit_class:=very_loose_fit to tight_fit do
        for fit_class in fit_class_kind::very_loose_fit as u8..=fit_class_kind::tight_fit as u8 {
            let fit_class = fit_class_kind::from(fit_class);
            // begin if minimal_demerits[fit_class]<=minimum_demerits then
            if $globals.minimal_demerits[fit_class] <= $globals.minimum_demerits {
                // @<Insert a new active node
                //   from |best_place[fit_class]| to |cur_p|@>;
                Insert_a_new_active_node_from_best_place_fit_class_to_cur_p!(
                    $globals,
                    $r,
                    $prev_r,
                    $break_type,
                    fit_class
                );
            }
            // minimal_demerits[fit_class]:=awful_bad;
            $globals.minimal_demerits[fit_class] = awful_bad;
            // end;
        }
        // minimum_demerits:=awful_bad;
        $globals.minimum_demerits = awful_bad;
        // @<Insert a delta node to prepare for the next active node@>;
        Insert_a_delta_node_to_prepare_for_the_next_active_node!(
            $globals,
            $r,
            $prev_r,
            $prev_prev_r
        );
        // end
        use crate::section_0817::fit_class_kind;
    }};
}
