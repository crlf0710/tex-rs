//! ` `

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1254(globals: &mut TeXGlobals) {
    // primitive("hyphenchar",assign_font_int,0);
    primitive(
        globals,
        crate::strpool_str!("hyphenchar"),
        assign_font_int,
        0,
    );
    // @!@:hyphen_char_}{\.{\\hyphenchar} primitive@>
    // primitive("skewchar",assign_font_int,1);
    primitive(globals, crate::strpool_str!("skewchar"), assign_font_int, 1);
    // @!@:skew_char_}{\.{\\skewchar} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0209::*;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
