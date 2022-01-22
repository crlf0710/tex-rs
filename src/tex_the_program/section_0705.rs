//! @ The |overbar| function returns a pointer to a vlist box that consists of
//! a given box |b|, above which has been placed a kern of height |k| under a
//! fraction rule of thickness |t| under additional space of height |t|.
//
// @p function overbar(@!b:pointer;@!k,@!t:scaled):pointer;
pub(crate) fn overbar(
    globals: &mut TeXGlobals,
    b: pointer,
    k: scaled,
    t: scaled,
) -> TeXResult<pointer> {
    // var p,@!q:pointer; {nodes being constructed}
    /// nodes being constructed
    let (mut p, q);
    // begin p:=new_kern(k); link(p):=b; q:=fraction_rule(t); link(q):=p;
    p = new_kern(globals, k)?;
    link!(globals, p) = b;
    q = fraction_rule(globals, t)?;
    link!(globals, q) = p;
    // p:=new_kern(t); link(p):=q; overbar:=vpack(p,natural);
    p = new_kern(globals, t)?;
    link!(globals, p) = q;
    let overbar = vpack(globals, p, natural0!(), natural1!())?;
    // end;
    crate::ok_nojump!(overbar)
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0156::new_kern;
use crate::section_0644::natural0;
use crate::section_0644::natural1;
use crate::section_0668::vpack;
use crate::section_0704::fraction_rule;
