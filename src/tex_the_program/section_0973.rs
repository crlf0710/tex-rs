//! ` `
// @<Use node |p| to update the current height and depth measurements...@>=
pub(crate) macro Use_node_p_to_update_the_current_height_and_depth_measurements_if_this_node_is_not_a_legal_breakpoint__goto_not_found_or_update_heights__otherwise_set_pi_to_the_associated_penalty_at_the_break($globals:expr, $p:expr, $pi:expr, $prev_p:expr, $prev_dp:expr, $lbl_update_heights:lifetime, $lbl_not_found:lifetime) {{
    // case type(p) of
    let type_p = r#type!($globals, $p);
    // hlist_node,vlist_node,rule_node: begin@t@>@;@/
    if type_p == hlist_node || type_p == vlist_node || type_p == rule_node {
        // cur_height:=cur_height+prev_dp+height(p); prev_dp:=depth(p);
        cur_height!($globals) += $prev_dp + height!($globals, $p);
        $prev_dp = depth!($globals, $p);
        // goto not_found;
        crate::goto_forward_label!($lbl_not_found);
        // end;
    }
    // whatsit_node:@<Process whatsit |p| in |vert_break| loop, |goto not_found|@>;
    else if type_p == whatsit_node {
        todo!("whatsit");
    }
    // glue_node: if precedes_break(prev_p) then pi:=0
    else if type_p == glue_node {
        if precedes_break!($globals, $prev_p) {
            $pi = 0;
        }
        // else goto update_heights;
        else {
            crate::goto_forward_label!($lbl_update_heights);
        }
    }
    // kern_node: begin if link(p)=null then t:=penalty_node
    else if type_p == kern_node {
        // else t:=type(link(p));
        // if t=glue_node then pi:=0@+else goto update_heights;
        // end;
        todo!("kern_node");
    }
    // penalty_node: pi:=penalty(p);
    else if type_p == penalty_node {
        $pi = penalty!($globals, $p);
    }
    // mark_node,ins_node: goto not_found;
    else if type_p == mark_node || type_p == ins_node {
        crate::goto_forward_label!($lbl_not_found);
    }
    // othercases confusion("vertbreak")
    else {
        confusion($globals, crate::strpool_str!("vertbreak"))?;
        // @:this can't happen vertbreak}{\quad vertbreak@>
    }
    // endcases
    use crate::section_0095::confusion;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::hlist_node;
    use crate::section_0137::vlist_node;
    use crate::section_0138::rule_node;
    use crate::section_0140::ins_node;
    use crate::section_0141::mark_node;
    use crate::section_0146::whatsit_node;
    use crate::section_0148::precedes_break;
    use crate::section_0149::glue_node;
    use crate::section_0155::kern_node;
    use crate::section_0157::penalty;
    use crate::section_0157::penalty_node;
    use crate::section_0970::cur_height;
}}
