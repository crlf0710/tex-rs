//! @ Files for \.{\\read} are opened and closed by the |in_stream| command.
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1272(globals: &mut TeXGlobals) {
    // primitive("openin",in_stream,1);
    primitive(globals, strpool_str!("openin"), in_stream, 1);
    // @!@:open_in_}{\.{\\openin} primitive@>
    // primitive("closein",in_stream,0);
    primitive(globals, strpool_str!("closein"), in_stream, 0);
    // @!@:close_in_}{\.{\\closein} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0208::in_stream;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
