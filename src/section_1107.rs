//! ` `

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1107(globals: &mut TeXGlobals) {
    // primitive("unpenalty",remove_item,penalty_node);@/
    // @!@:un_penalty_}{\.{\\unpenalty} primitive@>
    // primitive("unkern",remove_item,kern_node);@/
    // @!@:un_kern_}{\.{\\unkern} primitive@>
    // primitive("unskip",remove_item,glue_node);@/
    // @!@:un_skip_}{\.{\\unskip} primitive@>
    // primitive("unhbox",un_hbox,box_code);@/
    primitive(globals, strpool_str!("unhbox"), un_hbox, box_code as _);
    // @!@:un_hbox_}{\.{\\unhbox} primitive@>
    // primitive("unhcopy",un_hbox,copy_code);@/
    // @!@:un_hcopy_}{\.{\\unhcopy} primitive@>
    // primitive("unvbox",un_vbox,box_code);@/
    // @!@:un_vbox_}{\.{\\unvbox} primitive@>
    // primitive("unvcopy",un_vbox,copy_code);@/
    // @!@:un_vcopy_}{\.{\\unvcopy} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0208::un_hbox;
use crate::section_0264::primitive;
use crate::section_1071::box_code;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
