//! @ The |out_what| procedure takes care of outputting whatsit nodes for
//! |vlist_out| and |hlist_out|\kern-.3pt.
//
// @<Declare procedures needed in |hlist_out|, |vlist_out|@>=
// procedure out_what(@!p:pointer);
#[allow(unused_variables)]
pub(crate) fn out_what(globals: &mut TeXGlobals, p: pointer) -> TeXResult<()> {
    // var j:small_number; {write stream number}
    let subtype_p = subtype!(globals, p);
    // begin case subtype(p) of
    // open_node,write_node,close_node:@<Do some work that has been queued up
    //   for \.{\\write}@>;
    if subtype_p == open_node || subtype_p == write_node || subtype_p == close_node {
        Do_some_work_that_has_been_queued_up_for_write!(globals, p);
    }
    // special_node:special_out(p);
    else if subtype_p == special_node {
        todo!("special_out");
    }
    // language_node:do_nothing;
    else if subtype_p == language_node {
        do_nothing!();
    }
    // othercases confusion("ext4")
    else {
        confusion(globals, strpool_str!("ext4"))?;
    }
    // @:this can't happen ext4}{\quad ext4@>
    // endcases;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0095::confusion;
use crate::section_0115::pointer;
use crate::section_1341::*;
