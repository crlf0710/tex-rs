//! @ Just before doing the big |case| switch in the second pass, the program
//! sets up default values so that most of the branches are short.
//
// @<If node |q| is a style node, change the style...@>=
pub(crate) macro If_node_q_is_a_style_node__change_the_style_and_goto_delete_q__otherwise_if_it_is_not_a_noad__put_it_into_the_hlist__advance_q__and_goto_done__otherwise_set_s_to_the_size_of_noad_q__set_t_to_the_associated_type_ord_noad_to_inner_noad__and_set_pen_to_the_associated_penalty($globals:expr, $q:expr, $p:expr, $t:expr, $s:expr, $pen:expr, $lbl_done:lifetime) {{
    // t:=ord_noad; s:=noad_size; pen:=inf_penalty;
    $t = ord_noad;
    $s = noad_size;
    $pen = inf_penalty;
    // case type(q) of
    let type_q = r#type!($globals, $q);
    // op_noad,open_noad,close_noad,punct_noad,inner_noad: t:=type(q);
    if type_q == op_noad
        || type_q == open_noad
        || type_q == close_noad
        || type_q == punct_noad
        || type_q == inner_noad
    {
        $t = type_q;
    }
    // bin_noad: begin t:=bin_noad; pen:=bin_op_penalty;
    else if type_q == bin_noad {
        $t = bin_noad;
        $pen = bin_op_penalty!($globals);
        // end;
    }
    // rel_noad: begin t:=rel_noad; pen:=rel_penalty;
    else if type_q == rel_noad {
        $t = rel_noad;
        $pen = rel_penalty!($globals);
        // end;
    }
    // ord_noad,vcenter_noad,over_noad,under_noad: do_nothing;
    else if type_q == ord_noad
        || type_q == vcenter_noad
        || type_q == over_noad
        || type_q == under_noad
    {
        do_nothing!();
    }
    // radical_noad: s:=radical_noad_size;
    else if type_q == radical_noad {
        $s = radical_noad_size;
    }
    // accent_noad: s:=accent_noad_size;
    else if type_q == accent_noad {
        todo!("accent_noad");
    }
    // fraction_noad: s:=fraction_noad_size;
    else if type_q == fraction_noad {
        $s = fraction_noad_size;
    }
    // left_noad,right_noad: t:=make_left_right(q,style,max_d,max_h);
    else if type_q == left_noad || type_q == right_noad {
        todo!("left_noad || right_noad");
    }
    // style_node: @<Change the current style and |goto delete_q|@>;
    else if type_q == style_node {
        todo!("style_node");
    }
    // whatsit_node,penalty_node,rule_node,disc_node,adjust_node,ins_node,mark_node,
    //  glue_node,kern_node:@t@>@;@/
    else if type_q == whatsit_node
        || type_q == penalty_node
        || type_q == rule_node
        || type_q == disc_node
        || type_q == adjust_node
        || type_q == ins_node
        || type_q == mark_node
        || type_q == glue_node
        || type_q == kern_node
    {
        // begin link(p):=q; p:=q; q:=link(q); link(p):=null; goto done;
        link!($globals, $p) = $q;
        $p = $q;
        $q = link!($globals, $q);
        link!($globals, $p) = null;
        crate::goto_forward_label!($lbl_done);
        // end;
    }
    // othercases confusion("mlist3")
    else {
        crate::trace_error_expr!("type(q)={}", type_q);
        confusion($globals, crate::strpool_str!("mlist3"))?;
        // @:this can't happen mlist3}{\quad mlist3@>
    }
    // endcases
    use crate::section_0016::do_nothing;
    use crate::section_0095::confusion;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0138::rule_node;
    use crate::section_0140::ins_node;
    use crate::section_0141::mark_node;
    use crate::section_0142::adjust_node;
    use crate::section_0145::disc_node;
    use crate::section_0146::whatsit_node;
    use crate::section_0149::glue_node;
    use crate::section_0155::kern_node;
    use crate::section_0157::inf_penalty;
    use crate::section_0157::penalty_node;
    use crate::section_0236::bin_op_penalty;
    use crate::section_0236::rel_penalty;
    use crate::section_0681::noad_size;
    use crate::section_0682::*;
    use crate::section_0683::fraction_noad;
    use crate::section_0683::fraction_noad_size;
    use crate::section_0683::radical_noad;
    use crate::section_0683::radical_noad_size;
    use crate::section_0687::*;
    use crate::section_0688::style_node;
}}
