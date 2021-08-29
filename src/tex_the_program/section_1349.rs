//! @ Here is a subroutine that creates a whatsit node having a given |subtype|
//! and a given number of words. It initializes only the first word of the whatsit,
//! and appends it to the current list.
//
// @<Declare procedures needed in |do_extension|@>=
// procedure new_whatsit(@!s:small_number;@!w:small_number);
pub(crate) fn new_whatsit(
    globals: &mut TeXGlobals,
    s: small_number,
    w: small_number,
) -> TeXResult<()> {
    // var p:pointer; {the new node}
    /// the new node
    let p: pointer;
    // begin p:=get_node(w); type(p):=whatsit_node; subtype(p):=s;
    p = get_node(globals, w.get().into())?;
    r#type!(globals, p) = whatsit_node;
    subtype!(globals, p) = s.get();
    // link(tail):=p; tail:=p;
    link!(globals, tail!(globals)) = p;
    tail!(globals) = p;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0146::whatsit_node;

