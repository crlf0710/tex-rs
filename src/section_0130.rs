//! @ Conversely, when some variable-size node |p| of size |s| is no longer needed,
//! the operation |free_node(p,s)| will make its words available, by inserting
//! |p| as a new empty node just before where |rover| now points.
//! @^inner loop@>
//
// @p procedure free_node(@!p:pointer; @!s:halfword); {variable-size node
//   liberation}
/// variable-size node liberation
pub(crate) fn free_node(globals: &mut TeXGlobals, p: pointer, s: halfword) {
    // var q:pointer; {|llink(rover)|}
    /// `llink(rover)`
    let q: pointer;
    // begin node_size(p):=s; link(p):=empty_flag;
    node_size!(globals, p) = s;
    link!(globals, p) = empty_flag;
    // q:=llink(rover); llink(p):=q; rlink(p):=rover; {set both links}
    /// set both links
    const _ : () = ();
    q = llink!(globals, globals.rover);
    llink!(globals, p) = q;
    rlink!(globals, p) = globals.rover;
    // llink(rover):=p; rlink(q):=p; {insert |p| into the ring}
    /// insert `p` into the ring
    const _ : () = ();
    llink!(globals, globals.rover) = p;
    rlink!(globals, q) = p;
    // @!stat var_used:=var_used-s;@+tats@;{maintain statistics}
    region_stat!{
        /// maintain statistics
        const _ : () = ();
        globals.var_used -= s as integer;
        use crate::pascal::integer;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0113::halfword;
use crate::section_0115::pointer;
use crate::section_0124::empty_flag;
