//! @ One of the things we must do on the first pass is change a |bin_noad| to
//! an |ord_noad| if the |bin_noad| is not in the context of a binary operator.
//! The values of |r| and |r_type| make this fairly easy.
//
// @<Do first-pass processing...@>=
pub(crate) macro Do_first_pass_processing_based_on_type_q__goto_done_with_noad_if_a_noad_has_been_fully_processed__goto_check_dimensions_if_it_has_been_translated_into_new_hlist_q___or_goto_done_with_node_if_a_node_has_been_fully_processed($globals:expr, $q:expr, $delta:expr) {{
    // reswitch: delta:=0;
    $delta = 0;
    // case type(q) of
    let type_q = r#type!($globals, $q);
    // bin_noad: case r_type of
    if type_q == bin_noad {
        //   bin_noad,op_noad,rel_noad,open_noad,punct_noad,left_noad:
        //     begin type(q):=ord_noad; goto reswitch;
        //     end;
        //   othercases do_nothing
        //   endcases;
        // rel_noad,close_noad,punct_noad,right_noad: begin@t@>@;@/
        //   @<Convert \(a)a final |bin_noad| to an |ord_noad|@>;
        //   if type(q)=right_noad then goto done_with_noad;
        //   end;
        todo!("bin_noad");
    }
    // @t\4@>@<Cases for noads that can follow a |bin_noad|@>@;
    else if crate::section_0733::Cases_for_noads_that_can_follow_a_bin_noad!($globals, $q, type_q)
    {
        // already processed
    }
    // @t\4@>@<Cases for nodes that can appear in an mlist, after which we
    //   |goto done_with_node|@>@;
    else if false {
    }
    // othercases confusion("mlist1")
    else {
        crate::trace_error_expr!("type(q)={}", type_q);
        confusion($globals, crate::strpool_str!("mlist1"))?;
        // @:this can't happen mlist1}{\quad mlist1@>
    }
    // endcases;@/
    // @<Convert \(n)|nucleus(q)| to an hlist and attach the sub/superscripts@>
    todo!("Do first-pass");
    use crate::section_0095::confusion;
    use crate::section_0133::r#type;
    use crate::section_0682::bin_noad;
}}
