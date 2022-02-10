//! ` `

// @<Output leaders in a vlist...@>=
pub(crate) macro Output_leaders_in_a_vlist__goto_fin_rule_if_a_rule_or_to_next_p_if_done($globals:expr, $p:expr, $left_edge:expr, $top_edge:expr, $lbl_next_p:lifetime, $lbl_fin_rule:lifetime) {{
    /// the leader box being replicated
    let leader_box;
    /// height of leader box being replicated
    let leader_ht;
    // begin leader_box:=leader_ptr(p);
    leader_box = leader_ptr!($globals, $p);
    // if type(leader_box)=rule_node then
    if r#type!($globals, leader_box) == rule_node {
        // begin rule_wd:=width(leader_box); rule_dp:=0;
        $globals.rule_wd = width!($globals, leader_box);
        $globals.rule_dp = scaled::zero();
        // goto fin_rule;
        crate::goto_forward_label!($lbl_fin_rule);
        // end;
    }
    // leader_ht:=height(leader_box)+depth(leader_box);
    leader_ht = height!($globals, leader_box) + depth!($globals, leader_box);
    // if (leader_ht>0)and(rule_ht>0) then
    if leader_ht > scaled::zero() && $globals.rule_ht > scaled::zero() {
        /// bottom boundary of leader space
        let edge;
        /// extra space between leader boxes
        let mut lx;
        // begin rule_ht:=rule_ht+10; {compensate for floating-point rounding}
        /// compensate for floating-point rounding
        const _: () = ();
        $globals.rule_ht = scaled::new_from_inner($globals.rule_ht.inner() + 10);
        // edge:=cur_v+rule_ht; lx:=0;
        edge = $globals.cur_v + $globals.rule_ht;
        lx = scaled::zero();
        // @<Let |cur_v| be the position of the first box, and set |leader_ht+lx|
        //   to the spacing between corresponding parts of boxes@>;
        crate::section_0636::Let_cur_v_be_the_position_of_the_first_box__and_set_leader_ht_plus_lx_to_the_spacing_between_corresponding_parts_of_boxes!($globals, $p, $top_edge, lx, leader_ht);
        // while cur_v+leader_ht<=edge do
        while $globals.cur_v + leader_ht <= edge {
            // @<Output a leader box at |cur_v|,
            //   then advance |cur_v| by |leader_ht+lx|@>;
            crate::section_0637::Output_a_leader_box_at_cur_v__then_advance_cur_v_by_leader_ht_plus_lx!($globals, $left_edge, leader_box, lx, leader_ht);
        }
        // cur_v:=edge-10; goto next_p;
        $globals.cur_v = scaled::new_from_inner(edge.inner() - 10);
        crate::goto_forward_label!($lbl_next_p);
        // end;
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0138::rule_node;
    use crate::section_0149::leader_ptr;
}}
