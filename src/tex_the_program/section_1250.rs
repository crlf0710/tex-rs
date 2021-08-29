//! @ New hyphenation data is loaded by the |hyph_data| command.
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1250(globals: &mut TeXGlobals) {
    // primitive("hyphenation",hyph_data,0);
    primitive(globals, strpool_str!("hyphenation"), hyph_data, 0);
    // @!@:hyphenation_}{\.{\\hyphenation} primitive@>
    // primitive("patterns",hyph_data,1);
    primitive(globals, strpool_str!("patterns"), hyph_data, 1);
    // @!@:patterns_}{\.{\\patterns} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0209::hyph_data;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}


