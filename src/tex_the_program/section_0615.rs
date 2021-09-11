//! @ In case you are wondering when all the movement nodes are removed from
//! \TeX's memory, the answer is that they are recycled just before
//! |hlist_out| and |vlist_out| finish outputting a box. This restores the
//! down and right stacks to the state they were in before the box was output,
//! except that some |info|'s may have become more restrictive.
//
// @p procedure prune_movements(@!l:integer);
//   {delete movement nodes with |location>=l|}
/// delete movement nodes with `location>=l`
pub(crate) fn prune_movements(globals: &mut TeXGlobals, l: integer) {
    // label done,exit;
    // var p:pointer; {node being deleted}
    /// node being deleted
    let mut p: pointer;
    crate::region_forward_label!(
    |'done|
    {
    // begin while down_ptr<>null do
    while globals.down_ptr != null {
        // begin if location(down_ptr)<l then goto done;
        if location!(globals, globals.down_ptr) < l {
            crate::goto_forward_label!('done);
        }
        // p:=down_ptr; down_ptr:=link(p); free_node(p,movement_node_size);
        p = globals.down_ptr;
        globals.down_ptr = link!(globals, p);
        free_node(globals, p, movement_node_size as _);
        // end;
    }
    }
    // done: while right_ptr<>null do
    'done <-
    );
    while globals.right_ptr != null {
        // begin if location(right_ptr)<l then return;
        if location!(globals, globals.right_ptr) < l {
            return;
        }
        // p:=right_ptr; right_ptr:=link(p); free_node(p,movement_node_size);
        p = globals.right_ptr;
        globals.right_ptr = link!(globals, p);
        free_node(globals, p, movement_node_size as _);
        // end;
    }
    // exit:end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0130::free_node;
use crate::section_0605::location;
use crate::section_0605::movement_node_size;
