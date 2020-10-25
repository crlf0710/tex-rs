//! @ Either \.{\\dump} or \.{\\end} will cause |main_control| to enter the
//! endgame, since both of them have `|stop|' as their command code.
//!
//! @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1052(globals: &mut TeXGlobals) {
    // primitive("end",stop,0);@/
    // @!@:end_}{\.{\\end} primitive@>
    primitive(globals, strpool_str!("end"), stop, 0);
    // primitive("dump",stop,1);@/
    // @!@:dump_}{\.{\\dump} primitive@>
    primitive(globals, strpool_str!("dump"), stop, 1);
}

use crate::section_0004::TeXGlobals;
use crate::section_0207::*;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
