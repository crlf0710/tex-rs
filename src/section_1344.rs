//! @ Extensions might introduce new command codes; but it's best to use
//! |extension| with a modifier, whenever possible, so that |main_control|
//! stays the same.
//
// @d immediate_code=4 {command modifier for \.{\\immediate}}
/// command modifier for `\immediate`
pub(crate) const immediate_code: quarterword = 4;
// @d set_language_code=5 {command modifier for \.{\\setlanguage}}
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0334(globals: &mut TeXGlobals) {
    // primitive("openout",extension,open_node);@/
    // @!@:open_out_}{\.{\\openout} primitive@>
    // primitive("write",extension,write_node); write_loc:=cur_val;@/
    primitive(globals, strpool_str!("write"), extension, write_node as _);
    globals.write_loc = globals.cur_val as pointer;
    // @!@:write_}{\.{\\write} primitive@>
    // primitive("closeout",extension,close_node);@/
    // @!@:close_out_}{\.{\\closeout} primitive@>
    // primitive("special",extension,special_node);@/
    // @!@:special_}{\.{\\special} primitive@>
    // primitive("immediate",extension,immediate_code);@/
    primitive(globals, strpool_str!("immediate"), extension, immediate_code as _);
    // @!@:immediate_}{\.{\\immediate} primitive@>
    // primitive("setlanguage",extension,set_language_code);@/
    // @!@:set_language_}{\.{\\setlanguage} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0113::quarterword;
use crate::section_0115::pointer;
use crate::section_0208::extension;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use crate::section_1341::write_node;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
