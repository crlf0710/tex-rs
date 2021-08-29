//! ` `
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1277(globals: &mut TeXGlobals) {
    // primitive("message",message,0);
    primitive(globals, strpool_str!("message"), message, 0);
    // @!@:message_}{\.{\\message} primitive@>
    // primitive("errmessage",message,1);
    primitive(globals, strpool_str!("errmessage"), message, 1);
    // @!@:err_message_}{\.{\\errmessage} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0208::*;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
