//! @ When we get to this part of the code, the line from |r| to |cur_p| is
//! feasible, its badness is~|b|, and its fitness classification is |fit_class|.
//! We don't want to make an active node for this break yet, but we will
//! compute the total demerits and record them in the |minimal_demerits| array,
//! if such a break is the current champion among all ways to get to |cur_p|
//! in a given line-number class and fitness class.
//
// @<Record a new feasible break@>=
macro_rules! Record_a_new_feasible_break {
    ($globals:expr, $r:expr, $l:expr, $b:expr, $pi:expr, $break_type:expr, $fit_class:expr, $artificial_demerits:expr) => {{
        trace_span!("Record a new feasible break");
        /// demerits of test line
        let mut d: integer;

        // if artificial_demerits then d:=0
        if $artificial_demerits {
            d = 0;
        }
        // else @<Compute the demerits, |d|, from |r| to |cur_p|@>;
        else {
            Compute_the_demerits_d_from_r_to_cur_p!($globals, $r, d, $b, $pi, $break_type, $fit_class);
        }
        // @!stat if tracing_paragraphs>0 then
        region_stat! {
            if tracing_paragraphs!($globals) > 0 {
                // @<Print a symbolic description of this feasible break@>;
                Print_a_symbolic_description_of_this_feasible_break!
                    ($globals, $r, d, $b, $pi, $artificial_demerits);
            }
            // tats@;@/
        }
        // d:=d+total_demerits(r); {this is the minimum total demerits
        //   from the beginning to |cur_p| via |r|}
        d += total_demerits!($globals, $r);
        /// this is the minimum total demerits from the beginning to `cur_p` via `r`
        const _ : () = ();
        // if d<=minimal_demerits[fit_class] then
        if d <= $globals.minimal_demerits[$fit_class] {
            // begin minimal_demerits[fit_class]:=d;
            $globals.minimal_demerits[$fit_class] = d;
            // best_place[fit_class]:=break_node(r); best_pl_line[fit_class]:=l;
            $globals.best_place[$fit_class] = break_node!($globals, $r);
            $globals.best_pl_line[$fit_class] = $l;
            // if d<minimum_demerits then minimum_demerits:=d;
            if d < $globals.minimum_demerits {
                $globals.minimum_demerits = d;
            }
            // end
        }
    }}
}
