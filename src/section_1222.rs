//! @ A \.{\\chardef} creates a control sequence whose |cmd| is |char_given|;
//! a \.{\\mathchardef} creates a control sequence whose |cmd| is |math_given|;
//! and the corresponding |chr| is the character code or math code. A \.{\\countdef}
//! or \.{\\dimendef} or \.{\\skipdef} or \.{\\muskipdef} creates a control
//! sequence whose |cmd| is |assign_int| or \dots\ or |assign_mu_glue|, and the
//! corresponding |chr| is the |eqtb| location of the internal register in question.
//
// @d char_def_code=0 {|shorthand_def| for \.{\\chardef}}
/// `shorthand_def` for `\chardef`
pub(crate) const char_def_code: halfword = 0;
// @d math_char_def_code=1 {|shorthand_def| for \.{\\mathchardef}}
/// `shorthand_def` for `\mathchardef`
pub(crate) const math_char_def_code: halfword = 1;
// @d count_def_code=2 {|shorthand_def| for \.{\\countdef}}
/// `shorthand_def` for `\countdef`
pub(crate) const count_def_code: halfword = 2;
// @d dimen_def_code=3 {|shorthand_def| for \.{\\dimendef}}
// @d skip_def_code=4 {|shorthand_def| for \.{\\skipdef}}
// @d mu_skip_def_code=5 {|shorthand_def| for \.{\\muskipdef}}
// @d toks_def_code=6 {|shorthand_def| for \.{\\toksdef}}
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1222(globals: &mut TeXGlobals) {
    // primitive("chardef",shorthand_def,char_def_code);@/
    primitive(globals, strpool_str!("chardef"), shorthand_def, char_def_code);
    // @!@:char_def_}{\.{\\chardef} primitive@>
    // primitive("mathchardef",shorthand_def,math_char_def_code);@/
    primitive(globals, strpool_str!("mathchardef"), shorthand_def, math_char_def_code);
    // @!@:math_char_def_}{\.{\\mathchardef} primitive@>
    // primitive("countdef",shorthand_def,count_def_code);@/
    primitive(globals, strpool_str!("countdef"), shorthand_def, count_def_code);
    // @!@:count_def_}{\.{\\countdef} primitive@>
    // primitive("dimendef",shorthand_def,dimen_def_code);@/
    // @!@:dimen_def_}{\.{\\dimendef} primitive@>
    // primitive("skipdef",shorthand_def,skip_def_code);@/
    // @!@:skip_def_}{\.{\\skipdef} primitive@>
    // primitive("muskipdef",shorthand_def,mu_skip_def_code);@/
    // @!@:mu_skip_def_}{\.{\\muskipdef} primitive@>
    // primitive("toksdef",shorthand_def,toks_def_code);@/
    // @!@:toks_def_}{\.{\\toksdef} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

use crate::section_0209::shorthand_def;
use crate::section_0113::halfword;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
