//! @* \[49] Mode-independent processing.
//! The long |main_control| procedure has now been fully specified, except for
//! certain activities that are independent of the current mode. These activities
//! do not change the current vlist or hlist or mlist; if they change anything,
//! it is the value of a parameter or the meaning of a control sequence.
//!
//! Assignments to values in |eqtb| can be global or local. Furthermore, a
//! control sequence can be defined to be `\.{\\long}' or `\.{\\outer}', and
//! it might or might not be expanded. The prefixes `\.{\\global}', `\.{\\long}',
//! and `\.{\\outer}' can occur in any order. Therefore we assign binary numeric
//! codes, making it possible to accumulate the union of all specified prefixes
//! by adding the corresponding codes.  (\PASCAL's |set| operations could also
//! have been used.)

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1208(globals: &mut TeXGlobals) {
    // primitive("long",prefix,1);
    primitive(globals, crate::strpool_str!("long"), prefix, 1);
    // @!@:long_}{\.{\\long} primitive@>
    // primitive("outer",prefix,2);
    primitive(globals, crate::strpool_str!("outer"), prefix, 2);
    // @!@:outer_}{\.{\\outer} primitive@>
    // primitive("global",prefix,4);
    primitive(globals, crate::strpool_str!("global"), prefix, 4);
    // @!@:global_}{\.{\\global} primitive@>
    // primitive("def",def,0);
    primitive(globals, crate::strpool_str!("def"), def, 0);
    // @!@:def_}{\.{\\def} primitive@>
    // primitive("gdef",def,1);
    primitive(globals, crate::strpool_str!("gdef"), def, 1);
    // @!@:gdef_}{\.{\\gdef} primitive@>
    // primitive("edef",def,2);
    primitive(globals, crate::strpool_str!("edef"), def, 2);
    // @!@:edef_}{\.{\\edef} primitive@>
    // primitive("xdef",def,3);
    primitive(globals, crate::strpool_str!("xdef"), def, 3);
    // @!@:xdef_}{\.{\\xdef} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0209::*;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
