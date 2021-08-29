//! ` `

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1286(globals: &mut TeXGlobals) {
    // primitive("lowercase",case_shift,lc_code_base);
    primitive(globals, strpool_str!("lowercase"), case_shift, lc_code_base as _);
    // @!@:lowercase_}{\.{\\lowercase} primitive@>
    // primitive("uppercase",case_shift,uc_code_base);
    primitive(globals, strpool_str!("uppercase"), case_shift, uc_code_base as _);
    // @!@:uppercase_}{\.{\\uppercase} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0208::case_shift;
use crate::section_0230::lc_code_base;
use crate::section_0230::uc_code_base;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
