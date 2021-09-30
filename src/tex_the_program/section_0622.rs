//! ` `
// @<Output the non-|char_node| |p| for |hlist_out|...@>=
pub(crate) macro Output_the_non_char_node_p_for_hlist_out_and_move_to_the_next_node {
    ($globals:expr, $p:expr, $this_box:expr, $base_line:expr, $cur_glue:expr, $cur_g:expr, $g_sign:expr, $g_order:expr, $lbl_reswitch:lifetime) => {{
        // begin case type(p) of
        let type_p = r#type!($globals, $p);
        crate::region_forward_label!(
        |'next_p|
        {
        crate::region_forward_label!(
        |'move_past|
        {
        // hlist_node,vlist_node:@<Output a box in an hlist@>;
        if type_p == hlist_node || type_p == vlist_node {
            crate::section_0623::Output_a_box_in_an_hlist!($globals, $p, $base_line);
        }
        // rule_node: begin rule_ht:=height(p); rule_dp:=depth(p); rule_wd:=width(p);
        else if type_p == rule_node {
            // goto fin_rule;
            // end;
            todo!("rule_node in hlist");
        }
        // whatsit_node: @<Output the whatsit node |p| in an hlist@>;
        else if type_p == whatsit_node {
            todo!("whatsit_node in hlist");
        }
        // glue_node: @<Move right or output leaders@>;
        else if type_p == glue_node {
            crate::section_0625::Move_right_or_output_leaders!($globals, $p, $this_box, $cur_glue, $cur_g, $g_sign, $g_order, 'move_past);
        }
        // kern_node,math_node:cur_h:=cur_h+width(p);
        else if type_p == kern_node || type_p == math_node {
            $globals.cur_h += width!($globals, $p);
        }
        // ligature_node: @<Make node |p| look like a |char_node| and |goto reswitch|@>;
        else if type_p == ligature_node {
            crate::section_0652::Make_node_p_look_like_a_char_node_and_goto_reswitch!($globals, $p, crate::goto_forward_label, $lbl_reswitch);
        }
        // othercases do_nothing
        else {
            do_nothing!();
        }
        // endcases;@/
        // goto next_p;
        crate::goto_forward_label!('next_p);
        // fin_rule: @<Output a rule in an hlist@>;
        todo!("output a rule in hlist");
        }
        // move_past: cur_h:=cur_h+rule_wd;
        'move_past <-
        );
        $globals.cur_h += $globals.rule_wd;
        }
        // next_p:p:=link(p);
        'next_p <-
        );
        $p = link!($globals, $p);
        // end
        use crate::section_0016::do_nothing;
        use crate::section_0118::link;
        use crate::section_0133::r#type;
        use crate::section_0135::width;
        use crate::section_0135::hlist_node;
        use crate::section_0137::vlist_node;
        use crate::section_0138::rule_node;
        use crate::section_0143::ligature_node;
        use crate::section_0146::whatsit_node;
        use crate::section_0147::math_node;
        use crate::section_0149::glue_node;
        use crate::section_0155::kern_node;
    }}
}
