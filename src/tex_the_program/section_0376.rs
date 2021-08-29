//! @ The processing of \.{\\input} involves the |start_input| subroutine,
//! which will be declared later; the processing of \.{\\endinput} is trivial.
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0376(globals: &mut TeXGlobals) {
    // primitive("input",input,0);@/
    primitive(globals, strpool_str!("input"), input, 0);
    // @!@:input_}{\.{\\input} primitive@>
    // primitive("endinput",input,1);@/
    primitive(globals, strpool_str!("endinput"), input, 1);
    // @!@:end_input_}{\.{\\endinput} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0210::input;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
