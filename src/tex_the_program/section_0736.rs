//! ` `

// @<Declare math...@>=
// procedure make_vcenter(@!q:pointer);
pub(crate) fn make_vcenter(globals: &mut TeXGlobals, q: pointer) -> TeXResult<()> {
    // var v:pointer; {the box that should be centered vertically}
    /// the box that should be centered vertically
    let v;
    // @!delta:scaled; {its height plus depth}
    /// its height plus depth
    let delta;
    // begin v:=info(nucleus(q));
    v = info_inner!(globals, nucleus!(q));
    // if type(v)<>vlist_node then confusion("vcenter");
    if r#type!(globals, v) != vlist_node {
        confusion(globals, crate::strpool_str!("vcenter"))?;
    }
    // @:this can't happen vcenter}{\quad vcenter@>
    // delta:=height(v)+depth(v);
    delta = height!(globals, v) + depth!(globals, v);
    // height(v):=axis_height(cur_size)+half(delta);
    height!(globals, v) =
        axis_height!(globals, globals.cur_size.get()) + scaled::new_from_inner(half(delta.inner()));
    // depth(v):=delta-height(v);
    depth!(globals, v) = delta - height!(globals, v);
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_0100::half;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0118::info_inner;
use crate::section_0133::r#type;
use crate::section_0135::depth;
use crate::section_0135::height;
use crate::section_0137::vlist_node;
use crate::section_0681::nucleus;
use crate::section_0700::axis_height;
