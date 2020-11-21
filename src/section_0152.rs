//! @ And here's a function that creates a glue node for a given parameter
//! identified by its code number; for example,
//! |new_param_glue(line_skip_code)| returns a pointer to a glue node for the
//! current \.{\\lineskip}.
//
// @p function new_param_glue(@!n:small_number):pointer;
#[allow(unused_variables)]
pub(crate) fn new_param_glue(globals: &mut TeXGlobals, n: small_number) -> pointer {
    todo!();
    // var p:pointer; {the new node}
    // @!q:pointer; {the glue specification}
    // begin p:=get_node(small_node_size); type(p):=glue_node; subtype(p):=n+1;
    // leader_ptr(p):=null;@/
    // q:=@<Current |mem| equivalent of glue parameter number |n|@>@t@>;
    // glue_ptr(p):=q; incr(glue_ref_count(q));
    // new_param_glue:=p;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0101::small_number;
use crate::section_0115::pointer;

