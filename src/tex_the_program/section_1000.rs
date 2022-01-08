//! @ The title of this section is already so long, it seems best to avoid
//! making it more accurate but still longer, by mentioning the fact that a
//! kern node at the end of the contribution list will not be contributed until
//! we know its successor.
//
// @<If the current page is empty...@>=
pub(crate) macro If_the_current_page_is_empty_and_node_p_is_to_be_deleted__goto_done1__otherwise_use_node_p_to_update_the_state_of_the_current_page__if_this_node_is_an_insertion__goto_contribute__otherwise_if_this_node_is_not_a_legal_breakpoint__goto_contribute_or_update_heights__otherwise_set_pi_to_the_penalty_associated_with_this_breakpoint($globals:expr, $p:expr, $pi:expr, $lbl_continue:lifetime, $lbl_update_heights:lifetime, $lbl_contribute:lifetime, $lbl_done1:lifetime) {{
    // case type(p) of
    let type_p = r#type!($globals, $p);
    // hlist_node,vlist_node,rule_node: if page_contents<box_there then
    if type_p == hlist_node || type_p == vlist_node || type_p == rule_node {
        if $globals.page_contents < page_contents_kind::box_there {
            // @<Initialize the current page, insert the \.{\\topskip} glue
            //   ahead of |p|, and |goto continue|@>
            crate::section_1001::Initialize_the_current_page__insert_the_topskip_glue_ahead_of_p__and_goto_continue!(
                $globals,
                $p,
                $lbl_continue
            );
        }
        // else @<Prepare to move a box or rule node to the current page,
        //   then |goto contribute|@>;
        else {
            crate::section_1002::Prepare_to_move_a_box_or_rule_node_to_the_current_page__then_goto_contribute!(
                $globals,
                $p,
                $lbl_contribute
            );
        }
    }
    // whatsit_node: @<Prepare to move whatsit |p| to the current page,
    //   then |goto contribute|@>;
    else if type_p == whatsit_node {
        crate::section_1364::Prepare_to_move_whatsit_p_to_the_current_page__then_goto_contribute!($globals, $p, $lbl_contribute);
    }
    // glue_node: if page_contents<box_there then goto done1
    else if type_p == glue_node {
        if $globals.page_contents < page_contents_kind::box_there {
            crate::goto_forward_label!($lbl_done1);
        }
        // else if precedes_break(page_tail) then pi:=0
        else if precedes_break!($globals, $globals.page_tail) {
            $pi = 0;
        }
        // else goto update_heights;
        else {
            crate::goto_forward_label!($lbl_update_heights);
        }
    }
    // kern_node: if page_contents<box_there then goto done1
    else if type_p == kern_node {
        if $globals.page_contents < page_contents_kind::box_there {
            crate::goto_forward_label!($lbl_done1);
        }
        // else if link(p)=null then return
        else if link!($globals, $p) == null {
            crate::return_nojump!();
        }
        // else if type(link(p))=glue_node then pi:=0
        else if r#type!($globals, link!($globals, $p)) == glue_node {
            $pi = 0;
        }
        // else goto update_heights;
        else {
            crate::goto_forward_label!($lbl_update_heights);
        }
    }
    // penalty_node: if page_contents<box_there then goto done1@+else pi:=penalty(p);
    else if type_p == penalty_node {
        if $globals.page_contents < page_contents_kind::box_there {
            crate::goto_forward_label!($lbl_done1);
        } else {
            $pi = penalty!($globals, $p);
        }
    }
    // mark_node: goto contribute;
    else if type_p == mark_node {
        todo!("move mark_node");
    }
    // ins_node: @<Append an insertion to the current page and |goto contribute|@>;
    else if type_p == ins_node {
        crate::section_1008::Append_an_insertion_to_the_current_page_and_goto_contribute!($globals, $p, $lbl_contribute);
    }
    // othercases confusion("page")
    else {
        confusion($globals, crate::strpool_str!("page"))?;
        // @:this can't happen page}{\quad page@>
    }
    // endcases
    use crate::section_0095::confusion;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
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
    use crate::section_0980::page_contents_kind;
}}
