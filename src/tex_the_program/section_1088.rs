//! @ A paragraph begins when horizontal-mode material occurs in vertical mode,
//! or when the paragraph is explicitly started by `\.{\\indent}' or
//! `\.{\\noindent}'.
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1088(globals: &mut TeXGlobals) {
    // primitive("indent",start_par,1);
    primitive(globals, strpool_str!("indent"), start_par, 1);
    // @!@:indent_}{\.{\\indent} primitive@>
    // primitive("noindent",start_par,0);
    primitive(globals, strpool_str!("noindent"), start_par, 0);
    // @!@:no_indent_}{\.{\\noindent} primitive@>
}

use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0208::start_par;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
