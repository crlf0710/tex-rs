//! @ Anyone who has been reading the last few sections of the program will
//! be able to guess what comes next.
//
// @p function new_penalty(@!m:integer):pointer;
#[allow(unused_variables)]
pub(crate) fn new_penalty(globals: &mut TeXGlobals, m: integer) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p: pointer;
    // begin p:=get_node(small_node_size); type(p):=penalty_node;
    p = get_node(globals, small_node_size as _)?;
    r#type!(globals, p) = penalty_node;
    // subtype(p):=0; {the |subtype| is not used}
    /// the |subtype| is not used
    const _ : () = ();
    subtype!(globals, p) = 0;
    // penalty(p):=m; new_penalty:=p;
    penalty!(globals, p) = m;
    return_nojump!(p);
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0157::penalty_node;
use crate::section_0141::small_node_size;