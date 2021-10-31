//! ` `
//!
//! At the end of a math formula or subformula, the |fin_mlist| routine is
//! called upon to return a pointer to the newly completed mlist, and to
//! pop the nest back to the enclosing semantic level. The parameter to
//! |fin_mlist|, if not null, points to a |right_noad| that ends the
//! current mlist; this |right_noad| has not yet been appended.
//
// @<Declare the function called |fin_mlist|@>=
// function fin_mlist(@!p:pointer):pointer;
pub(crate) fn fin_mlist(globals: &mut TeXGlobals, p: pointer) -> pointer {
    // var q:pointer; {the mlist to return}
    /// the mlist to return
    let q;
    // begin if incompleat_noad<>null then @<Compleat the incompleat noad@>
    if incompleat_noad!(globals) as pointer != null {
        todo!("Compleat the incompleat noad");
    }
    // else  begin link(tail):=p; q:=link(head);
    else {
        link!(globals, tail!(globals)) = p;
        q = link!(globals, head!(globals));
        // end;
    }
    // pop_nest; fin_mlist:=q;
    pop_nest(globals);
    return q;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0213::head;
use crate::section_0213::incompleat_noad;
use crate::section_0213::tail;
use crate::section_0217::pop_nest;
