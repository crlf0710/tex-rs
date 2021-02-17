//! ` `
// @<Output the non-|char_node| |p| for |vlist_out|@>=
macro_rules! Output_the_non_char_node_p_for_vlist_out {
    ($globals:expr, $p:expr, $left_edge:expr, $lbl_next_p:lifetime) => {{
        // begin case type(p) of
        let type_p = r#type!($globals, $p);
        // hlist_node,vlist_node:@<Output a box in a vlist@>;
        if type_p == hlist_node || type_p == vlist_node {
            Output_a_box_in_a_vlist!($globals, $p, $left_edge);
        }
        // rule_node: begin rule_ht:=height(p); rule_dp:=depth(p); rule_wd:=width(p);
        else if type_p == rule_node {
            todo!("rule node");
            // goto fin_rule;
            // end;
        }
        // whatsit_node: @<Output the whatsit node |p| in a vlist@>;
        else if type_p == whatsit_node {
            todo!("output whatsit node");
        }
        // glue_node: @<Move down or output leaders@>;
        else if type_p == glue_node {
            todo!("move down or output leaders");
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
        goto_forward_label!($lbl_next_p);
        // fin_rule: @<Output a rule in a vlist, |goto next_p|@>;
        todo!("fin_rule");
        // move_past: cur_v:=cur_v+rule_ht;
        todo!("update cur_v");
        // end
        use crate::section_0135::hlist_node;
        use crate::section_0137::vlist_node;
        use crate::section_0138::rule_node;
        use crate::section_0146::whatsit_node;
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
    }}
}
