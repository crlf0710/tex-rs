//! @ And here's a function that creates a glue node for a given parameter
//! identified by its code number; for example,
//! |new_param_glue(line_skip_code)| returns a pointer to a glue node for the
//! current \.{\\lineskip}.
//
// @p function new_param_glue(@!n:small_number):pointer;
#[allow(unused_variables)]
pub(crate) fn new_param_glue(globals: &mut TeXGlobals, n: small_number) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p: pointer;
    // @!q:pointer; {the glue specification}
    /// the glue specification
    let q: pointer;
    // begin p:=get_node(small_node_size); type(p):=glue_node; subtype(p):=n+1;
    p = get_node(globals, small_node_size.into())?;
    r#type!(globals, p) = glue_node;
    subtype!(globals, p) = (n.get() + 1) as _;
    // leader_ptr(p):=null;@/
    leader_ptr!(globals, p) = null;
    // q:=@<Current |mem| equivalent of glue parameter number |n|@>@t@>;
    q = Current_mem_equivalent_of_glue_parameter_number_n!(globals, n.get());
    // glue_ptr(p):=q; incr(glue_ref_count(q));
    glue_ptr!(globals, p) = q;
    incr!(glue_ref_count!(globals, q));
    // new_param_glue:=p;
    ok_nojump!(p)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0125::get_node;
use crate::section_0141::small_node_size;
use crate::section_0149::glue_node;
