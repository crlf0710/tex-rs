//! @ A subtle point to be noted here is that the maximum depth~|d| might be
//! negative, so |cur_height| and |prev_dp| might need to be corrected even
//! after a glue or kern node.
//
// @<If node |p| is a legal breakpoint, check...@>=
pub(crate) macro If_node_p_is_a_legal_breakpoint__check_if_this_break_is_the_best_known__and_goto_done_if_p_is_null_or_if_the_page_so_far_is_already_too_full_to_accept_more_stuff($globals:expr, $p:expr, $h:expr, $d:expr, $pi:expr, $prev_p:expr, $prev_dp:expr, $best_place:expr, $least_cost:expr, $lbl_done:lifetime) {{
    crate::region_forward_label! {
        |'not_found|
        {
            crate::region_forward_label! {
                |'update_heights|
                {
                    // if p=null then pi:=eject_penalty
                    if $p == null {
                        $pi = eject_penalty;
                    }
                    // else  @<Use node |p| to update the current height and depth measurements;
                    //  if this node is not a legal breakpoint, |goto not_found|
                    //  or |update_heights|,
                    //  otherwise set |pi| to the associated penalty at the break@>;
                    else {
                        crate::section_0973::Use_node_p_to_update_the_current_height_and_depth_measurements_if_this_node_is_not_a_legal_breakpoint__goto_not_found_or_update_heights__otherwise_set_pi_to_the_associated_penalty_at_the_break!($globals, $p, $pi, $prev_p, $prev_dp, 'update_heights, 'not_found);
                    }
                    // @<Check if node |p| is a new champion breakpoint; then \(go)|goto done|
                    //   if |p| is a forced break or if the page-so-far is already too full@>;
                    crate::section_0974::Check_if_node_p_is_a_new_champion_breakpoint__then_goto_done_if_p_is_a_forced_break_or_if_the_page_so_far_is_already_too_full!($globals, $p, $h, $pi, $prev_dp, $best_place, $least_cost, $lbl_done);
                    // if (type(p)<glue_node)or(type(p)>kern_node) then goto not_found;
                    if r#type!($globals, $p) < glue_node || r#type!($globals, $p) > kern_node {
                        crate::goto_forward_label!('not_found);
                    }
                }
                // update_heights: @<Update the current height and depth measurements with
                //   respect to a glue or kern node~|p|@>;
                'update_heights <-
            }
            crate::section_0976::Update_the_current_height_and_depth_measurements_with_respect_to_a_glue_or_kern_node_p!($globals, $p, $prev_dp);
        }
        // not_found: if prev_dp>d then
        'not_found <-
    }
    if $prev_dp > $d {
        // begin cur_height:=cur_height+prev_dp-d;
        cur_height!($globals) += $prev_dp - $d;
        // prev_dp:=d;
        $prev_dp = $d;
        // end;
    }
    use crate::section_0115::null;
    use crate::section_0133::r#type;
    use crate::section_0149::glue_node;
    use crate::section_0155::kern_node;
    use crate::section_0157::eject_penalty;
    use crate::section_0970::cur_height;
}}
