//! ` `

// @<Cases for nodes that can appear in an mlist...@>=
pub(crate) macro Cases_for_nodes_that_can_appear_in_an_mlist__after_which_we_goto_done_with_node($globals:expr, $q:expr, $type_q:expr, $lbl_done_with_node:lifetime) {{
    // style_node: begin cur_style:=subtype(q);
    let processed = if $type_q == style_node {
        $globals.cur_style = subtype!($globals, $q).into();
        // @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
        crate::section_0703::Set_up_the_values_of_cur_size_and_cur_mu__based_on_cur_style!(
            $globals
        );
        // goto done_with_node;
        crate::goto_forward_label!($lbl_done_with_node);
        // end;
        true
    }
    // choice_node: @<Change this node to a style node followed by the correct choice,
    //    then |goto done_with_node|@>;
    else if $type_q == choice_node {
        crate::section_0731::Change_this_node_to_a_style_node_followed_by_the_correct_choice__then_goto_done_with_node!(
            $globals,
            $q,
            $lbl_done_with_node
        );
        true
    }
    // ins_node,mark_node,adjust_node,
    //   whatsit_node,penalty_node,disc_node: goto done_with_node;
    else if $type_q == ins_node
        || $type_q == mark_node
        || $type_q == adjust_node
        || $type_q == whatsit_node
        || $type_q == penalty_node
        || $type_q == disc_node
    {
        todo!("ins_node, ...");
        true
    }
    // rule_node: begin if height(q)>max_h then max_h:=height(q);
    //   if depth(q)>max_d then max_d:=depth(q); goto done_with_node;
    //   end;
    else if $type_q == rule_node {
        todo!("rule_node");
        true
    }
    // glue_node: begin @<Convert \(m)math glue to ordinary glue@>;
    else if $type_q == glue_node {
        crate::section_0732::Convert_math_glue_to_ordinary_glue!($globals, $q);
        // goto done_with_node;
        crate::goto_forward_label!($lbl_done_with_node);
        // end;
        true
    }
    // kern_node: begin math_kern(q,cur_mu); goto done_with_node;
    else if $type_q == kern_node {
        math_kern($globals, $q, $globals.cur_mu);
        crate::goto_forward_label!($lbl_done_with_node);
        // end;
        true
    } else {
        false
    };
    use crate::section_0133::subtype;
    use crate::section_0138::rule_node;
    use crate::section_0140::ins_node;
    use crate::section_0141::mark_node;
    use crate::section_0142::adjust_node;
    use crate::section_0145::disc_node;
    use crate::section_0146::whatsit_node;
    use crate::section_0149::glue_node;
    use crate::section_0155::kern_node;
    use crate::section_0157::penalty_node;
    use crate::section_0688::style_node;
    use crate::section_0689::choice_node;
    use crate::section_0717::math_kern;
    processed
}}
