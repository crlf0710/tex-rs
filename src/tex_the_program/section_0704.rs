//! @ Here is a function that returns a pointer to a rule node having a given
//! thickness |t|. The rule will extend horizontally to the boundary of the vlist
//! that eventually contains it.
//
// @p function fraction_rule(@!t:scaled):pointer;
//   {construct the bar for a fraction}
/// construct the bar for a fraction
pub(crate) fn fraction_rule(globals: &mut TeXGlobals, t: scaled) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p;
    // begin p:=new_rule; height(p):=t; depth(p):=0; fraction_rule:=p;
    p = new_rule(globals)?;
    height!(globals, p) = t;
    depth!(globals, p) = scaled::zero();
    // end;
    crate::ok_nojump!(p)
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0135::depth;
use crate::section_0135::height;
use crate::section_0139::new_rule;
