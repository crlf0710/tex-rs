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
pub(crate) fn copy_node_list(globals: &mut TeXGlobals, mut p: pointer) -> TeXResult<pointer> {
    // var h:pointer; {temporary head of copied list}
    /// temporary head of copied list
    let h;
    // @!q:pointer; {previous position in new list}
    /// previous position in new list
    let mut q;
    // @!r:pointer; {current node being fabricated for new list}
    /// current node being fabricated for new list
    let mut r;
    // @!words:0..5; {number of words remaining to be copied}
    // begin h:=get_avail; q:=h;
    h = get_avail(globals);
    q = h;
    // while p<>null do
    while p != null {
        // begin @<Make a copy of node |p| in node |r|@>;
        crate::section_0205::Make_a_copy_of_node_p_in_node_r!(globals, p, r);
        // link(q):=r; q:=r; p:=link(p);
        link!(globals, q) = r;
        q = r;
        p = link!(globals, p);
        // end;
    }
    // link(q):=null; q:=link(h); free_avail(h);
    link!(globals, q) = null;
    q = link!(globals, h);
    free_avail!(globals, h);
    // copy_node_list:=q;
    crate::ok_nojump!(q)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0120::get_avail;
use crate::section_0121::free_avail;
