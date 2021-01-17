//! @ The copying procedure copies words en masse without bothering
//! to look at their individual fields. If the node format changes---for
//! example, if the size is altered, or if some link field is moved to another
//! relative position---then this code may need to be changed too.
//! @^data structure assumptions@>
//
// @p function copy_node_list(@!p:pointer):pointer; {makes a duplicate of the
//   node list that starts at |p| and returns a pointer to the new list}
/// makes a duplicate of the node list that starts at `p` and returns a pointer to the new list
#[allow(unused_variables)]
pub(crate) fn copy_node_list(globals: &mut TeXGlobals, p: pointer) -> pointer {
    // var h:pointer; {temporary head of copied list}
    // @!q:pointer; {previous position in new list}
    // @!r:pointer; {current node being fabricated for new list}
    // @!words:0..5; {number of words remaining to be copied}
    // begin h:=get_avail; q:=h;
    // while p<>null do
    //   begin @<Make a copy of node |p| in node |r|@>;
    //   link(q):=r; q:=r; p:=link(p);
    //   end;
    // link(q):=null; q:=link(h); free_avail(h);
    // copy_node_list:=q;
    // end;
    todo!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
