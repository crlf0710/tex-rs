//! @ Glue nodes that are more or less anonymous are created by |new_glue|,
//! whose argument points to a glue specification.

#[non_exhaustive]
enum glue_node_subtype {
    normal = 0,
}

// @p function new_glue(@!q:pointer):pointer;
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn new_glue(globals: &mut TeXGlobals, q: pointer) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p: pointer;
    // begin p:=get_node(small_node_size); type(p):=glue_node; subtype(p):=normal;
    p = get_node(globals, small_node_size as _)?;
    r#type!(globals, p) = glue_node;
    subtype!(globals, p) = normal as _;
    // leader_ptr(p):=null; glue_ptr(p):=q; incr(glue_ref_count(q));
    leader_ptr!(globals, p) = null;
    glue_ptr!(globals, p) = q;
    incr!(glue_ref_count!(globals, q));
    // new_glue:=p;
    ok_nojump!(p)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0125::get_node;
use crate::section_0141::small_node_size;
use crate::section_0149::glue_node;
use glue_node_subtype::*;

