//! @ The procedure |flush_list(p)| frees an entire linked list of
//! one-word nodes that starts at position |p|.
//! @^inner loop@>
//
// @p procedure flush_list(@!p:pointer); {makes list of single-word nodes
//   available}
/// makes list of single-word nodes available
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn flush_list(globals: &mut TeXGlobals, p: pointer) {
    // var @!q,@!r:pointer; {list traversers}
    /// list traversers
    let (mut q, mut r) : (pointer, pointer);
    // begin if p<>null then
    if p != null {
        // begin r:=p;
        r = p;
        // repeat q:=r; r:=link(r); @!stat decr(dyn_used);@+tats@/
        loop {
            q = r;
            r = link!(globals, r);
            region_stat! {
                decr!(globals.dyn_used);
            }
            // until r=null; {now |q| is the last node on the list}
            if r == null {
                /// now `q` is the last node on the list
                break;
            }
        }
        // link(q):=avail; avail:=p;
        link!(globals, q) = globals.avail;
        globals.avail = p;
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0115::null;
