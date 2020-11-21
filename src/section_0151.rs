//! @ Here is a function that returns a pointer to a copy of a glue spec.
//! The reference count in the copy is |null|, because there is assumed
//! to be exactly one reference to the new specification.
//
// @p function new_spec(@!p:pointer):pointer; {duplicates a glue specification}
/// duplicates a glue specification
pub(crate) fn new_spec(globals: &mut TeXGlobals, p: pointer) -> Result<pointer, JumpOutToEndOfTEX> {
    // var q:pointer; {the new spec}
    /// the new spec
    let q: pointer;
    // begin q:=get_node(glue_spec_size);@/
    q = get_node(globals, glue_spec_size as _)?;
    // mem[q]:=mem[p]; glue_ref_count(q):=null;@/
    globals.mem[q] = globals.mem[p];
    glue_ref_count!(globals, q) = null;
    // width(q):=width(p); stretch(q):=stretch(p); shrink(q):=shrink(p);
    width!(globals, q) = width!(globals, p);
    stretch!(globals, q) = stretch!(globals, p);
    shrink!(globals, q) = shrink!(globals, p);
    // new_spec:=q;
    ok_nojump!(q)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::JumpOutToEndOfTEX;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0125::get_node;
use crate::section_0150::glue_spec_size;

