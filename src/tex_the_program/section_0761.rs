//! @ Just before doing the big |case| switch in the second pass, the program
//! sets up default values so that most of the branches are short.
//
// @<If node |q| is a style node, change the style...@>=
pub(crate) macro If_node_q_is_a_style_node__change_the_style_and_goto_delete_q__otherwise_if_it_is_not_a_noad__put_it_into_the_hlist__advance_q__and_goto_done__otherwise_set_s_to_the_size_of_noad_q__set_t_to_the_associated_type_ord_noad_to_inner_noad__and_set_pen_to_the_associated_penalty($globals:expr, $q:expr, $t:expr, $s:expr, $pen:expr) {{
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
    //   end;
    // rel_noad: begin t:=rel_noad; pen:=rel_penalty;
    //   end;
    // ord_noad,vcenter_noad,over_noad,under_noad: do_nothing;
    else if type_q == ord_noad
        || type_q == vcenter_noad
        || type_q == over_noad
        || type_q == under_noad
    {
        do_nothing!();
    }
    // radical_noad: s:=radical_noad_size;
    // accent_noad: s:=accent_noad_size;
    // fraction_noad: s:=fraction_noad_size;
    // left_noad,right_noad: t:=make_left_right(q,style,max_d,max_h);
    // style_node: @<Change the current style and |goto delete_q|@>;
    // whatsit_node,penalty_node,rule_node,disc_node,adjust_node,ins_node,mark_node,
    //  glue_node,kern_node:@t@>@;@/
    //   begin link(p):=q; p:=q; q:=link(q); link(p):=null; goto done;
    //   end;
    // othercases confusion("mlist3")
    else {
        crate::trace_error_expr!("type(q)={}", type_q);
        confusion($globals, crate::strpool_str!("mlist3"))?;
        // @:this can't happen mlist3}{\quad mlist3@>
    }
    // endcases
    use crate::section_0016::do_nothing;
    use crate::section_0095::confusion;
    use crate::section_0133::r#type;
    use crate::section_0157::inf_penalty;
    use crate::section_0681::noad_size;
    use crate::section_0682::*;
    use crate::section_0687::*;
}}
