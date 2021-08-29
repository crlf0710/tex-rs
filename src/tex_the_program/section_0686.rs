//! @ The |new_noad| function creates an |ord_noad| that is completely null.

#[doc(hidden)]
#[derive(Clone, Copy)]
pub(crate) enum noad_subtype {
    normal = 0,
}

// @p function new_noad:pointer;
pub(crate) fn new_noad(globals: &mut TeXGlobals) -> TeXResult<pointer> {
    // var p:pointer;
    let p: pointer;
    // begin p:=get_node(noad_size);
    p = get_node(globals, noad_size as _)?;
    // type(p):=ord_noad; subtype(p):=normal;
    r#type!(globals, p) = ord_noad;
    subtype!(globals, p) = noad_subtype::normal as _;
    // mem[nucleus(p)].hh:=empty_field;
    globals.mem[nucleus!(p)][MEMORY_WORD_HH] = empty_field;
    // mem[subscr(p)].hh:=empty_field;
    globals.mem[subscr!(p)][MEMORY_WORD_HH] = empty_field;
    // mem[supscr(p)].hh:=empty_field;
    globals.mem[supscr!(p)][MEMORY_WORD_HH] = empty_field;
    // new_noad:=p;
    Ok(p)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::MEMORY_WORD_HH;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0681::noad_size;
use crate::section_0682::ord_noad;
use crate::section_0684::empty_field;
