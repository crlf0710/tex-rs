//! @ The procedure |flush_list(p)| frees an entire linked list of
//! one-word nodes that starts at position |p|.
//! @^inner loop@>
//
// @p procedure flush_list(@!p:pointer); {makes list of single-word nodes
//   available}
/// makes list of single-word nodes available
#[allow(unused_variables)]
#[cfg_attr(feature = "trace_verbose", tracing::instrument(level = "trace"))]
pub(crate) fn flush_list(globals: &mut TeXGlobals, p: pointer) {
    // var @!q,@!r:pointer; {list traversers}
    /// list traversers
    let (mut q, mut r): (pointer, pointer);
    // begin if p<>null then
    if p != null {
        // begin r:=p;
        r = p;
        // repeat q:=r; r:=link(r); @!stat decr(dyn_used);@+tats@/
        loop {
            q = r;
            r = link!(globals, r);
            crate::region_stat! {
                decr!(globals.dyn_used);
                use crate::section_0016::decr;
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
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
