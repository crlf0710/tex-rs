//! @ Both \.{\\let} and \.{\\futurelet} share the command code |let|.
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1219(globals: &mut TeXGlobals) {
    // primitive("let",let,normal);@/
    primitive(globals, strpool_str!("let"), r#let, let_kind::normal as _);
    // @!@:let_}{\.{\\let} primitive@>
    // primitive("futurelet",let,normal+1);@/
    primitive(globals, strpool_str!("futurelet"), r#let,let_kind::futurelet as _);
    // @!@:future_let_}{\.{\\futurelet} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use crate::section_0209::*;
use crate::section_0135::let_kind;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}

