//! @ The |new_kern| function creates a kern node having a given width.

// @p function new_kern(@!w:scaled):pointer;
pub(crate) fn new_kern(globals: &mut TeXGlobals, w: scaled) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p: pointer;
    // begin p:=get_node(small_node_size); type(p):=kern_node;
    p = get_node(globals, small_node_size.into())?;
    r#type!(globals, p) = kern_node;
    // subtype(p):=normal;
    subtype!(globals, p) = kern_node_subtype::normal as _;
    // width(p):=w;
    width!(globals, p) = w;
    // new_kern:=p;
    crate::ok_nojump!(p)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0133::r#type;
use crate::section_0133::subtype;
use crate::section_0135::width;
use crate::section_0141::small_node_size;
use crate::section_0155::kern_node;
use crate::section_0155::kern_node_subtype;
