//! @ A new rule node is delivered by the |new_rule| function. It
//! makes all the dimensions ``running,'' so you have to change the
//! ones that are not allowed to run.
//
// @p function new_rule:pointer;
pub(crate) fn new_rule(globals: &mut TeXGlobals) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p: pointer;
    // begin p:=get_node(rule_node_size); type(p):=rule_node;
    p = get_node(globals, rule_node_size as _)?;
    r#type!(globals, p) = rule_node;
    // subtype(p):=0; {the |subtype| is not used}
    /// the `subtype` is not used
    const _: () = ();
    subtype!(globals, p) = 0;
    // width(p):=null_flag; depth(p):=null_flag; height(p):=null_flag;
    width!(globals, p) = null_flag;
    depth!(globals, p) = null_flag;
    height!(globals, p) = null_flag;
    // new_rule:=p;
    return_nojump!(p);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0138::null_flag;
use crate::section_0138::rule_node;
use crate::section_0138::rule_node_size;
