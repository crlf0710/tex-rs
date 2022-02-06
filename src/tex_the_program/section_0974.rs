//! ` `

// @d deplorable==100000 {more than |inf_bad|, but less than |awful_bad|}
/// more than `inf_bad`, but less than `awful_bad`
pub(crate) const deplorable: integer = 100000;

// @<Check if node |p| is a new champion breakpoint; then \(go)...@>=
pub(crate) macro Check_if_node_p_is_a_new_champion_breakpoint__then_goto_done_if_p_is_a_forced_break_or_if_the_page_so_far_is_already_too_full($globals:expr, $p:expr, $h:expr, $pi:expr, $prev_dp:expr, $best_place:expr, $least_cost:expr, $lbl_done:lifetime) {{
    // if pi<inf_penalty then
    if $pi < inf_penalty {
        /// badness at a trial breakpoint
        let mut b: integer;
        // begin @<Compute the badness, |b|, using |awful_bad|
        //   if the box is too full@>;
        crate::section_0975::Compute_the_badness_b__using_awful_bad_if_the_box_is_too_full!(
            $globals, b, $h
        );
        // if b<awful_bad then
        if b < awful_bad {
            // if pi<=eject_penalty then b:=pi
            if $pi <= eject_penalty {
                b = $pi;
            }
            // else if b<inf_bad then b:=b+pi
            else if b < inf_bad as integer {
                b += $pi;
            }
            // else b:=deplorable;
            else {
                b = deplorable;
            }
        }
        // if b<=least_cost then
        if b <= $least_cost {
            // begin best_place:=p; least_cost:=b;
            $best_place = $p;
            $least_cost = b;
            // best_height_plus_depth:=cur_height+prev_dp;
            $globals.best_height_plus_depth = cur_height!($globals) + $prev_dp;
            // end;
        }
        // if (b=awful_bad)or(pi<=eject_penalty) then goto done;
        if b == awful_bad || $pi < eject_penalty {
            crate::goto_forward_label!($lbl_done);
        }
        // end
    }
    use crate::section_0108::inf_bad;
    use crate::section_0157::eject_penalty;
    use crate::section_0157::inf_penalty;
    use crate::section_0833::awful_bad;
    use crate::section_0970::cur_height;
}}

use crate::pascal::integer;
