//! @ One of the things we must do on the first pass is change a |bin_noad| to
//! an |ord_noad| if the |bin_noad| is not in the context of a binary operator.
//! The values of |r| and |r_type| make this fairly easy.
//
// @<Do first-pass processing...@>=
pub(crate) macro Do_first_pass_processing_based_on_type_q__goto_done_with_noad_if_a_noad_has_been_fully_processed__goto_check_dimensions_if_it_has_been_translated_into_new_hlist_q___or_goto_done_with_node_if_a_node_has_been_fully_processed($globals:expr, $q:expr, $r:expr, $r_type:expr, $delta:expr, $lbl_check_dimensions:lifetime, $lbl_done_with_noad:lifetime, $lbl_done_with_node:lifetime) {{
    // reswitch: delta:=0;
    crate::region_backward_label!{
        'reswitch <-
        {
            $delta = scaled::zero();
            // case type(q) of
            let type_q = r#type!($globals, $q);
            // bin_noad: case r_type of
            if type_q == bin_noad {
                // bin_noad,op_noad,rel_noad,open_noad,punct_noad,left_noad:
                if $r_type == bin_noad || $r_type == op_noad || $r_type == rel_noad || $r_type == open_noad || $r_type == punct_noad ||$r_type == left_noad {
                    // begin type(q):=ord_noad; goto reswitch;
                    r#type!($globals, $q) = ord_noad;
                    crate::goto_backward_label!('reswitch);
                    // end;
                }
                // othercases do_nothing
                else {
                    do_nothing!();
                }
                // endcases;
            // rel_noad,close_noad,punct_noad,right_noad: begin@t@>@;@/
            } else if type_q == rel_noad || type_q == close_noad || type_q == punct_noad || type_q == right_noad {
                // @<Convert \(a)a final |bin_noad| to an |ord_noad|@>;
                crate::section_0729::Convert_a_final_bin_noad_to_an_ord_noad!($globals, $r, $r_type);
                // if type(q)=right_noad then goto done_with_noad;
                if r#type!($globals, $q) == right_noad {
                    crate::goto_forward_label!($lbl_done_with_noad);
                }
                // end;
            }
            // @t\4@>@<Cases for noads that can follow a |bin_noad|@>@;
            else if crate::section_0733::Cases_for_noads_that_can_follow_a_bin_noad!($globals, $q, type_q, $delta, $lbl_check_dimensions, $lbl_done_with_noad)
            {
                // already processed
            }
            // @t\4@>@<Cases for nodes that can appear in an mlist, after which we
            //   |goto done_with_node|@>@;
            else if crate::section_0730::Cases_for_nodes_that_can_appear_in_an_mlist__after_which_we_goto_done_with_node!(
                $globals, $q, type_q, $lbl_done_with_node
            ) {
                // already processed
            }
            // othercases confusion("mlist1")
            else {
                crate::trace_error_expr!("type(q)={}", type_q);
                confusion($globals, crate::strpool_str!("mlist1"))?;
                // @:this can't happen mlist1}{\quad mlist1@>
            }
            // endcases;@/
            // @<Convert \(n)|nucleus(q)| to an hlist and attach the sub/superscripts@>
            crate::section_0754::Convert_nucleus_q_to_an_hlist_and_attach_the_sub_superscripts!(
                $globals,
                $q,
                $delta,
                $lbl_check_dimensions
            );
        }
        |'reswitch|
    }
    use crate::section_0016::do_nothing;
    use crate::section_0095::confusion;
    use crate::section_0101::scaled;
    use crate::section_0133::r#type;
    use crate::section_0682::*;
    use crate::section_0687::*;
}}
