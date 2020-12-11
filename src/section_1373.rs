//! @ The |out_what| procedure takes care of outputting whatsit nodes for
//! |vlist_out| and |hlist_out|\kern-.3pt.
//
// @<Declare procedures needed in |hlist_out|, |vlist_out|@>=
// procedure out_what(@!p:pointer);
#[allow(unused_variables)]
pub(crate) fn out_what(globals: &mut TeXGlobals, p: pointer) {
    todo!();
    // var j:small_number; {write stream number}
    // begin case subtype(p) of
    // open_node,write_node,close_node:@<Do some work that has been queued up
    //   for \.{\\write}@>;
    // special_node:special_out(p);
    // language_node:do_nothing;
    // othercases confusion("ext4")
    // @:this can't happen ext4}{\quad ext4@>
    // endcases;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
