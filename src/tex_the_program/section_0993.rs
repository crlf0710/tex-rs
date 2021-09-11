//! @ The following procedure guarantees that a given box register
//! does not contain an \.{\\hbox}.
//
// @p procedure ensure_vbox(@!n:eight_bits);
pub(crate) fn ensure_vbox(globals: &mut TeXGlobals, n: eight_bits) -> TeXResult<()> {
    // var p:pointer; {the box register contents}
    /// the box register contents
    let p: pointer;
    // begin p:=box(n);
    p = r#box!(globals, n);
    // if p<>null then if type(p)=hlist_node then
    if p != null && r#type!(globals, p) == hlist_node {
        // begin print_err("Insertions can only be added to a vbox");
        // @.Insertions can only...@>
        // help3("Tut tut: You're trying to \insert into a")@/
        //   ("\box register that now contains an \hbox.")@/
        //   ("Proceed, and I'll discard its present contents.");
        // box_error(n);
        // end;
        todo!("box error");
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0025::eight_bits;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0133::r#type;
use crate::section_0135::hlist_node;
use crate::section_0230::r#box;
