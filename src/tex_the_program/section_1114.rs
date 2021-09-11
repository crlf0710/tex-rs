//! @ Discretionary nodes are easy in the common case `\.{\\-}', but in the
//! general case we must process three braces full of items.
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1114(globals: &mut TeXGlobals) {
    // primitive("-",discretionary,1);
    primitive(globals, crate::strpool_str!("-"), discretionary, 1);
    // @!@:Single-character primitives -}{\quad\.{\\-}@>
    // primitive("discretionary",discretionary,0);
    primitive(
        globals,
        crate::strpool_str!("discretionary"),
        discretionary,
        0,
    );
    // @!@:discretionary_}{\.{\\discretionary} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0208::discretionary;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
