//! @ Here's the way we sometimes want to display a token list, given a pointer
//! to its reference count; the pointer may be null.
//
// @p procedure token_show(@!p:pointer);
#[allow(unused_variables)]
pub(crate) fn token_show(globals: &mut TeXGlobals, p:pointer) {
    // begin if p<>null then show_token_list(link(p),null,10000000);
    if p != null {
        /// show_token_list(link(p), null, 10000000);
        todo!();
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::null;
use crate::section_0115::pointer;