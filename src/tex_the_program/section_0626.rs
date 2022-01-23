//! ` `
// @<Output leaders in an hlist...@>=
pub(crate) macro Output_leaders_in_an_hlist__goto_fin_rule_if_a_rule_or_to_next_p_if_done($globals:expr, $p:expr, $base_line:expr, $left_edge:expr, $lbl_next_p:lifetime) {{
    /// the leader box being replicated
    let leader_box;
    /// width of leader box being replicated
    let leader_wd;
    // begin leader_box:=leader_ptr(p);
    leader_box = leader_ptr!($globals, $p);
    // if type(leader_box)=rule_node then
    if r#type!($globals, leader_box) == rule_node {
        // begin rule_ht:=height(leader_box); rule_dp:=depth(leader_box);
        // goto fin_rule;
        // end;
        todo!("rule_node");
    }
    // leader_wd:=width(leader_box);
    leader_wd = width!($globals, leader_box);
    // if (leader_wd>0)and(rule_wd>0) then
    if leader_wd > scaled::zero() && $globals.rule_wd > scaled::zero() {
        /// left edge of sub-box, or right edge of leader space
        let edge;
        /// extra space between leader boxes
        let mut lx;
        // begin rule_wd:=rule_wd+10; {compensate for floating-point rounding}
        /// compensate for floating-point rounding
        const _: () = ();
        $globals.rule_wd = scaled::new_from_inner($globals.rule_wd.inner() + 10);
        // edge:=cur_h+rule_wd; lx:=0;
        edge = $globals.cur_h + $globals.rule_wd;
        lx = scaled::zero();
        // @<Let |cur_h| be the position of the first box, and set |leader_wd+lx|
        //   to the spacing between corresponding parts of boxes@>;
        crate::section_0627::Let_cur_h_be_the_position_of_the_first_box__and_set_leader_wd_plus_lx_to_the_spacing_between_corresponding_parts_of_boxes!($globals, $p, $left_edge, leader_wd, lx);
        // while cur_h+leader_wd<=edge do
        while $globals.cur_h + leader_wd <= edge {
            // @<Output a leader box at |cur_h|,
            //   then advance |cur_h| by |leader_wd+lx|@>;
            crate::section_0628::Output_a_leader_box_at_cur_h__then_advance_cur_h_by_leader_wd_plus_lx!($globals, $base_line, leader_box, leader_wd, lx);
        }
        // cur_h:=edge-10; goto next_p;
        $globals.cur_h = scaled::new_from_inner(edge.inner() - 10);
        crate::goto_forward_label!($lbl_next_p);
        // end;
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0133::r#type;
    use crate::section_0135::width;
    use crate::section_0138::rule_node;
    use crate::section_0149::leader_ptr;
}}
