//! @ A subroutine called |prune_page_top| takes a pointer to a vlist and
//! returns a pointer to a modified vlist in which all glue, kern, and penalty nodes
//! have been deleted before the first box or rule node. However, the first
//! box or rule is actually preceded by a newly created glue node designed so that
//! the topmost baseline will be at distance |split_top_skip| from the top,
//! whenever this is possible without backspacing.
//!
//! In this routine and those that follow, we make use of the fact that a
//! vertical list contains no character nodes, hence the |type| field exists
//! for each node in the list.
//! @^data structure assumptions@>
//
// @p function prune_page_top(@!p:pointer):pointer; {adjust top after page break}
/// adjust top after page break
pub(crate) fn prune_page_top(globals: &mut TeXGlobals, mut p: pointer) -> TeXResult<pointer> {
    // var prev_p:pointer; {lags one step behind |p|}
    /// lags one step behind `p`
    let mut prev_p;
    // @!q:pointer; {temporary variable for list manipulation}
    /// temporary variable for list manipulation
    let mut q;
    // begin prev_p:=temp_head; link(temp_head):=p;
    prev_p = temp_head;
    link!(globals, temp_head) = p;
    // while p<>null do
    while p != null {
        // case type(p) of
        let type_p = r#type!(globals, p);
        // hlist_node,vlist_node,rule_node:@<Insert glue for |split_top_skip|
        //   and set~|p:=null|@>;
        if type_p == hlist_node || type_p == vlist_node || type_p == rule_node {
            crate::section_0969::Insert_glue_for_split_top_skip_and_set_p_to_null!(
                globals, p, q, prev_p
            );
        }
        // whatsit_node,mark_node,ins_node: begin prev_p:=p; p:=link(prev_p);
        else if type_p == whatsit_node || type_p == mark_node || type_p == ins_node {
            prev_p = p;
            p = link!(globals, prev_p);
            // end;
        }
        // glue_node,kern_node,penalty_node: begin q:=p; p:=link(q); link(q):=null;
        else if type_p == glue_node || type_p == kern_node || type_p == penalty_node {
            q = p;
            p = link!(globals, q);
            link!(globals, q) = null;
            // link(prev_p):=p; flush_node_list(q);
            link!(globals, prev_p) = p;
            flush_node_list(globals, q)?;
            // end;
        }
        // othercases confusion("pruning")
        else {
            confusion(globals, crate::strpool_str!("pruning"))?;
            // @:this can't happen pruning}{\quad pruning@>
        }
        // endcases;
    }
    // prune_page_top:=link(temp_head);
    let prune_page_top = link!(globals, temp_head);
    // end;
    crate::ok_nojump!(prune_page_top)
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0133::r#type;
use crate::section_0135::hlist_node;
use crate::section_0137::vlist_node;
use crate::section_0138::rule_node;
use crate::section_0140::ins_node;
use crate::section_0141::mark_node;
use crate::section_0146::whatsit_node;
use crate::section_0149::glue_node;
use crate::section_0155::kern_node;
use crate::section_0157::penalty_node;
use crate::section_0162::temp_head;
use crate::section_0202::flush_node_list;
