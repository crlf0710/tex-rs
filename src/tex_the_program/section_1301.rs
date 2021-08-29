// @ @<Initialize table entries...@>=
// format_ident:=" (INITEX)";
#[distributed_slice(INIT_TBLENTRY)]
fn initialize_table_entries_done_by_initex_only_1301(globals: &mut TeXGlobals) {
    globals.format_ident = strpool_str!(" (INITEX)");
}

use crate::section_0004::TeXGlobals;
use crate::section_0008::INIT_TBLENTRY;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
