//! @ Here's the way we sometimes want to display a token list, given a pointer
//! to its reference count; the pointer may be null.
//
// @p procedure token_show(@!p:pointer);
#[allow(unused_variables)]
#[cfg_attr(feature = "trace_verbose", tracing::instrument(level = "trace"))]
pub(crate) fn token_show(globals: &mut TeXGlobals, p: pointer) {
    // begin if p<>null then show_token_list(link(p),null,10000000);
    if p != null {
        show_token_list(globals, link!(globals, p) as _, null as _, 10000000);
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0292::show_token_list;
