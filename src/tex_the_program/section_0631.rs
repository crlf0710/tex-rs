//! ` `
// @<Output the non-|char_node| |p| for |vlist_out|@>=
pub(crate) macro Output_the_non_char_node_p_for_vlist_out {
    ($globals:expr, $p:expr, $left_edge:expr, $top_edge:expr, $this_box:expr, $cur_glue:expr, $cur_g:expr, $g_sign:expr, $g_order:expr, $lbl_next_p:lifetime) => {{
        // begin case type(p) of
        let type_p = r#type!($globals, $p);
        crate::region_forward_label!(
        |'move_past|
        {
        crate::region_forward_label!(
        |'fin_rule|
        {
        // hlist_node,vlist_node:@<Output a box in a vlist@>;
        if type_p == hlist_node || type_p == vlist_node {
            crate::section_0632::Output_a_box_in_a_vlist!($globals, $p, $left_edge);
        }
        // rule_node: begin rule_ht:=height(p); rule_dp:=depth(p); rule_wd:=width(p);
        else if type_p == rule_node {
            $globals.rule_ht = height!($globals, $p);
            $globals.rule_dp = depth!($globals, $p);
            $globals.rule_wd = width!($globals, $p);
            // goto fin_rule;
            crate::goto_forward_label!('fin_rule);
            // end;
        }
        // whatsit_node: @<Output the whatsit node |p| in a vlist@>;
        else if type_p == whatsit_node {
            crate::section_1366::Output_the_whatsit_node_p_in_a_vlist!($globals, $p);
        }
        // glue_node: @<Move down or output leaders@>;
        else if type_p == glue_node {
            crate::section_0634::Move_down_or_output_leaders!($globals, $p, $left_edge, $top_edge, $this_box, $cur_glue, $cur_g, $g_sign, $g_order, 'move_past, $lbl_next_p, 'fin_rule);
        }
        // kern_node:cur_v:=cur_v+width(p);
        else if type_p == kern_node {
            $globals.cur_v += width!($globals, $p);
        }
        // othercases do_nothing
        else {
            do_nothing!();
        }
        // endcases;@/
        // goto next_p;
        crate::goto_forward_label!($lbl_next_p);
        // fin_rule: @<Output a rule in a vlist, |goto next_p|@>;
        }
        'fin_rule <-
        );
        crate::section_0633::Output_a_rule_in_a_vlist__goto_next_p!($globals, $this_box, $lbl_next_p);
        }
        // move_past: cur_v:=cur_v+rule_ht;
        'move_past <-
        );
        $globals.cur_v += $globals.rule_ht;
        // end
        use crate::section_0016::do_nothing;
        use crate::section_0133::r#type;
        use crate::section_0135::width;
        use crate::section_0135::height;
        use crate::section_0135::depth;
        use crate::section_0135::hlist_node;
        use crate::section_0137::vlist_node;
        use crate::section_0138::rule_node;
        use crate::section_0146::whatsit_node;
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
    }}
}
