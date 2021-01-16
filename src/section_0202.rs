//! @ Now we are ready to delete any node list, recursively.
//! In practice, the nodes deleted are usually charnodes (about 2/3 of the time),
//! and they are glue nodes in about half of the remaining cases.
//! @^recursion@>
//
// @p procedure flush_node_list(@!p:pointer); {erase list of nodes starting at |p|}
/// erase list of nodes starting at `p`
pub(crate) fn flush_node_list(globals: &mut TeXGlobals, mut p: pointer) -> TeXResult<()> {
    // label done; {go here when node |p| has been freed}
    // var q:pointer; {successor to node |p|}
    // begin while p<>null do
    while p != null {
        /// successor to node `p`
        let q: pointer;
        
        // @^inner loop@>
        // begin q:=link(p);
        q = link!(globals, p);
        // if is_char_node(p) then free_avail(p)
        if is_char_node!(globals, p) {
            free_avail!(globals, p);
        }
        // else  begin case type(p) of
        else {
            region_forward_label!(
            |'done|
            {
                let type_p = r#type!(globals, p);
                if false {
                    unreachable!();
                }
                // hlist_node,vlist_node,unset_node: begin flush_node_list(list_ptr(p));
                else if type_p == hlist_node || type_p == vlist_node || type_p == unset_node {
                    flush_node_list(globals, list_ptr!(globals, p))?;
                    // free_node(p,box_node_size); goto done;
                    free_node(globals, p, box_node_size as _);
                    goto_forward_label!('done);
                    // end;
                }
                // rule_node: begin free_node(p,rule_node_size); goto done;
                //   end;
                // ins_node: begin flush_node_list(ins_ptr(p));
                //   delete_glue_ref(split_top_ptr(p));
                //   free_node(p,ins_node_size); goto done;
                //   end;
                // whatsit_node: @<Wipe out the whatsit node |p| and |goto done|@>;
                else if type_p == whatsit_node {
                    Wipe_out_the_whatsit_node_p_and_goto_done!(globals, p, 'done);
                }
                // glue_node: begin fast_delete_glue_ref(glue_ptr(p));
                else if type_p == glue_node {
                    let glue_p = glue_ptr!(globals, p);
                    fast_delete_glue_ref!(globals, glue_p);
                    // if leader_ptr(p)<>null then flush_node_list(leader_ptr(p));
                    if leader_ptr!(globals, p) != null {
                        flush_node_list(globals, leader_ptr!(globals, p))?;
                    }
                    // end;
                }
                // kern_node,math_node,penalty_node: do_nothing;
                // ligature_node: flush_node_list(lig_ptr(p));
                // mark_node: delete_token_ref(mark_ptr(p));
                // disc_node: begin flush_node_list(pre_break(p));
                //   flush_node_list(post_break(p));
                //   end;
                // adjust_node: flush_node_list(adjust_ptr(p));
                // @t\4@>@<Cases of |flush_node_list| that arise in mlists only@>@;
                // othercases confusion("flushing")
                else {
                    trace_error_expr!("type(p)={}", type_p);
                    confusion(globals, strpool_str!("flushing"))?;
                }
                // @:this can't happen flushing}{\quad flushing@>
                // endcases;@/
                // free_node(p,small_node_size);
                free_node(globals, p, small_node_size as _);
            }
            // done:end;
            'done <-
            );
        }
        // p:=q;
        p = q;
        // end;
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0130::free_node;
use crate::section_0135::hlist_node;
use crate::section_0135::box_node_size;
use crate::section_0137::vlist_node;
use crate::section_0141::small_node_size;
use crate::section_0146::whatsit_node;
use crate::section_0149::glue_node;
use crate::section_0159::unset_node;